use std::{
    fmt::{self, Debug},
    mem::swap,
    path::PathBuf,
    sync::{Arc, Barrier},
    thread,
};

use crate::{cell::Cell, vtk_writer::VtkWriter, withcall::WithCall, VtkOutput};
use parking_lot::{Mutex, RwLock};

type NeighbourGrid<T> = Vec<Vec<Vec<Option<Arc<RwLock<T>>>>>>;

pub struct Grid<F, T>
where
    T: Clone + VtkOutput,
{
    pub grid: Arc<Vec<Vec<Arc<RwLock<T>>>>>,
    pub nb_grid: NeighbourGrid<T>,
    op: WithCall<F, T>,
    ext: (usize, usize),
    runners: usize,
    dimension: (usize, usize),
    steps: usize,
    output_steps: usize,
    neighbours: Vec<(i8, i8)>,
    vtk_writer: Arc<Mutex<VtkWriter<T>>>,
    some_val: T,
}

impl<F, T> Grid<F, T>
where
    F: Fn(&T, Vec<Option<&T>>) -> T + Clone + Send + Copy,
    T: Clone + Send + Sync + VtkOutput,
{
    fn compute_number_of_block_rows(number_of_processes: usize) -> usize {
        let mut number_of_rows = (number_of_processes as f32).sqrt() as usize;
        while number_of_processes % number_of_rows != 0 {
            number_of_rows -= 1;
        }
        number_of_rows
    }

    pub fn new(
        dimension: (usize, usize),
        op: WithCall<F, T>,
        runners: usize,
        height: impl Fn(usize, usize) -> T,
        steps: usize,
        output_steps: usize,
        mut neighbours: Vec<(i8, i8)>,
        output_path: PathBuf,
    ) -> Result<Self, InvalidThreadNumber> {
        neighbours.iter_mut().for_each(|(x, y)| swap(x, y));

        let dimension = (dimension.1, dimension.0);

        if dimension.0 * dimension.1 % runners != 0 {
            return Err(InvalidThreadNumber {});
        }

        let number_of_blocks_y = Self::compute_number_of_block_rows(runners);
        let number_of_blocks_x = runners / number_of_blocks_y;

        let x_ext = dimension.0 / number_of_blocks_x;
        let y_ext = dimension.1 / number_of_blocks_y;

        let mut grid = Vec::with_capacity(dimension.0);
        let mut gridn = Vec::with_capacity(dimension.0);

        for _ in 0..dimension.0 {
            grid.push(Vec::with_capacity(dimension.1));
        }
        for _ in 0..dimension.0 {
            gridn.push(Vec::with_capacity(dimension.1));
        }

        for row in gridn.iter_mut() {
            for _ in 0..dimension.1 {
                row.push(Vec::with_capacity(neighbours.len()));
            }
        }

        for (x, row) in grid.iter_mut().enumerate() {
            for y in 0..dimension.1 {
                row.push(Arc::new(RwLock::new(height(y, x))));
            }
        }

        let writer = VtkWriter::new(output_path, T::value_names());

        let mut s = Self {
            grid: Arc::new(grid),
            nb_grid: gridn,
            op,
            ext: (x_ext, y_ext),
            runners,
            dimension,
            steps,
            output_steps,
            neighbours,
            vtk_writer: Arc::new(Mutex::new(writer)),
            some_val: height(0, 0),
        };

        s.populate();

        Ok(s)
    }

    pub fn populate(&mut self) {
        for i in 0..self.dimension.0 {
            for j in 0..self.dimension.1 {
                let mut arr = Vec::with_capacity(self.neighbours.len());
                for offset in self.neighbours.iter() {
                    let nb_pos = (i as i32 + offset.0 as i32, j as i32 + offset.1 as i32);
                    let nb = if nb_pos.0 < 0
                        || nb_pos.1 < 0
                        || nb_pos.0 >= self.dimension.0 as i32
                        || nb_pos.1 >= self.dimension.1 as i32
                    {
                        None
                    } else {
                        Option::from((self.grid[nb_pos.0 as usize][nb_pos.1 as usize]).clone())
                    };
                    arr.push(nb);
                }
                self.nb_grid[i][j] = arr;
            }
        }
    }

    pub fn calculate(&mut self) {
        let sync_lock = Arc::new(Barrier::new(self.runners));
        let start_lock = Arc::new(Barrier::new(self.runners));
        let write_lock = Arc::new(Barrier::new(self.runners));

        thread::scope(|scope| {
            let mut running_x = 0;
            let mut running_y = 0;
            let mut counter = 0;

            let every_n_steps = self.steps / self.output_steps;

            for rank in 0..self.runners {
                let op1 = self.op.clone();
                let start_lock = start_lock.clone();
                let sync_lock = sync_lock.clone();
                let write_lock = write_lock.clone();
                let mut cells: Vec<Cell<T>> = vec![];
                let grid = self.grid.clone();
                let dimension = self.dimension;
                let steps = self.steps;
                let writer = self.vtk_writer.clone();
                let some_val = self.some_val.clone();

                for x in 0..self.ext.0 {
                    for y in 0..self.ext.1 {
                        cells.push(Cell {
                            value: self.grid[running_x + x][running_y + y].clone(),
                            neighbours: std::mem::take(
                                &mut self.nb_grid[running_x + x][running_y + y],
                            ),
                            next_val: some_val.clone(),
                        });
                    }
                }
                running_x += self.ext.0;

                if running_x >= self.dimension.0 {
                    running_x = 0;
                    running_y += self.ext.1;
                }

                scope.spawn(move || {
                    for _ in 0..steps {
                        start_lock.wait();
                        for cell in cells.iter_mut() {
                            cell.run(&op1);
                        }
                        sync_lock.wait();
                        for cell in cells.iter_mut() {
                            cell.write();
                        }
                        write_lock.wait();
                        if rank == 0 {
                            counter += 1;
                            if counter == every_n_steps {
                                counter = 0;

                                let mut out: Vec<Vec<T>> =
                                    vec![vec![some_val.clone(); dimension.1]; dimension.0];
                                for x in 0..dimension.0 {
                                    for y in 0..dimension.1 {
                                        out[x][y] = grid[x][y].read().clone();
                                    }
                                }
                                {
                                    let w = &mut *writer.lock();
                                    w.write_step(out);
                                }
                            }
                        }
                    }
                });
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct InvalidThreadNumber();

impl fmt::Display for InvalidThreadNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Size of domain (x*y) was not divisible by the number of threads"
        )
    }
}
