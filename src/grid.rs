use std::{
    mem::replace,
    sync::{atomic::Ordering, Arc, Barrier, Mutex},
    thread,
};

use crate::{
    cell::{AtomicF64, Cell},
    writer::Writer,
    WithCall, OUT_STEPS, SIZE, STEPS,
};

const EMPTY_OPTIONS: [Option<Arc<AtomicF64>>; 4] =
    [Option::None, Option::None, Option::None, Option::None];

pub struct Grid<F> {
    pub grid: Arc<Vec<Vec<Arc<AtomicF64>>>>,
    pub nb_grid: Vec<Vec<[Option<Arc<AtomicF64>>; 4]>>,
    op: WithCall<F>,
    ext: (usize, usize),
    runners: usize,
    dimension: usize,
    writer: Arc<Mutex<Writer>>,
}

impl<F> Grid<F>
where
    F: Fn(f64, &[Option<f64>]) -> f64 + Clone + std::marker::Send + Copy,
{
    fn compute_number_of_block_rows(number_of_processes: usize) -> usize {
        let mut number_of_rows = (number_of_processes as f32).sqrt() as usize;
        while number_of_processes % number_of_rows != 0 {
            number_of_rows -= 1;
        }
        number_of_rows
    }

    pub fn new(
        dimension: usize,
        op: WithCall<F>,
        runners: usize,
        height: impl Fn(usize, usize) -> f64,
    ) -> Self {
        if dimension * dimension % runners != 0 {
            panic!("dimension x dimension must be divisible by num_runners");
        }

        let number_of_blocks_y = Self::compute_number_of_block_rows(runners);
        let number_of_blocks_x = runners / number_of_blocks_y;

        let x_ext = dimension / number_of_blocks_x;
        let y_ext = dimension / number_of_blocks_y;

        let mut grid = Vec::with_capacity(dimension);
        let mut gridn = Vec::with_capacity(dimension);

        for _ in 0..dimension {
            grid.push(Vec::with_capacity(dimension));
        }
        for _ in 0..dimension {
            gridn.push(Vec::with_capacity(dimension));
        }

        for row in gridn.iter_mut() {
            for _ in 0..dimension {
                row.push(EMPTY_OPTIONS);
            }
        }

        for (x, row) in grid.iter_mut().enumerate() {
            for y in 0..dimension {
                row.push(Arc::new(AtomicF64::new(height(x, y))));
            }
        }

        Self {
            grid: Arc::new(grid),
            nb_grid: gridn,
            op,
            ext: (x_ext, y_ext),
            runners,
            dimension,
            writer: Arc::new(Mutex::new(Writer::new())),
        }
    }

    pub fn populate(&mut self) {
        const OFFSETS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        for i in 0..SIZE {
            for j in 0..SIZE {
                for (idx, offset) in OFFSETS.iter().enumerate() {
                    let nb_pos = (i as i32 + offset.0 as i32, j as i32 + offset.1 as i32);
                    let nb = if nb_pos.0 < 0
                        || nb_pos.1 < 0
                        || nb_pos.0 >= SIZE as i32
                        || nb_pos.1 >= SIZE as i32
                    {
                        None
                    } else {
                        Option::from((self.grid[nb_pos.0 as usize][nb_pos.1 as usize]).clone())
                    };
                    self.nb_grid[i][j][idx] = nb;
                }
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

            // let mut out_grid = vec![];
            // let dimension = self.dimension;

            // for _ in 0..self.dimension {
            //     out_grid.push(Vec::with_capacity(self.dimension));
            // }
            // for (x, row) in out_grid.iter_mut().enumerate() {
            //     for y in 0..self.dimension {
            //         row.push(self.grid[x][y].clone());
            //     }
            // }

            let every_n_steps = STEPS / OUT_STEPS;

            for rank in 0..self.runners {
                let op1 = self.op.clone();
                let start_lock = start_lock.clone();
                let sync_lock = sync_lock.clone();
                let write_lock = write_lock.clone();
                let mut cells: Vec<Cell> = vec![];
                let writer = self.writer.clone();
                let grid = self.grid.clone();
                let dimension = self.dimension;

                for x in 0..self.ext.0 {
                    for y in 0..self.ext.1 {
                        cells.push(Cell {
                            value: self.grid[running_x + x][running_y + y].clone(),
                            neighbours: replace(
                                &mut self.nb_grid[running_x + x][running_y + y],
                                EMPTY_OPTIONS,
                            ),
                            next_val: 0.0,
                        });
                    }
                }
                running_x += self.ext.0;

                if running_x >= self.dimension {
                    running_x = 0;
                    running_y += self.ext.1;
                }

                scope.spawn(move || {
                    for i in 0..STEPS {
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
                            // println!("entered");
                            if counter == every_n_steps {
                                counter = 0;

                                // println!("output written at step {}", i);

                                let mut out: Vec<Vec<f64>> = vec![vec![0.0; dimension]; dimension];

                                for x in 0..dimension {
                                    for y in 0..dimension {
                                        out[x][y] = grid[x][y].load(Ordering::Acquire);
                                    }
                                }

                                writer.lock().unwrap().write(out);
                            }
                        }
                    }
                });
            }
        });
    }

    // fn write(&mut self) {
    //     let mut out: Vec<Vec<f64>> = vec![];

    //     for x in 0..self.dimension {
    //         for y in 0..self.dimension {
    //             out[x][y] = self.grid[x][y].load(Ordering::Acquire);
    //         }
    //     }

    //     self.writer.write(out);
    // }

    pub fn print(&self) {
        for v in self.grid.iter() {
            for c in v {
                print!("{}, ", c.load(Ordering::Acquire));
            }
            println!();
        }
    }
}
