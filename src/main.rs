use grid::Grid;
use parking_lot::RwLockReadGuard;

mod cell;
mod grid;
mod writer;

#[derive(Clone)]
struct WithCall<F, T>
where
    T: Clone,
{
    // TODO fix this, this does not need clone
    fp: F,
    _type: Option<T>, //this is a hacky workaround to make the below impl block work
}

impl<F, T> WithCall<F, T>
where
    F: Fn(RwLockReadGuard<T>, &Vec<Option<RwLockReadGuard<T>>>) -> T,
    T: Clone,
{
    pub fn new(fp: F) -> Self {
        WithCall {
            fp,
            _type: Option::None,
        }
    }

    pub fn run(&self, a: RwLockReadGuard<T>, b: &Vec<Option<RwLockReadGuard<T>>>) -> T {
        (self.fp)(a, b)
    }
}

fn main() {
    let size = (3, 3);

    let op = WithCall::new(
        |num: RwLockReadGuard<f64>, nb: &Vec<Option<RwLockReadGuard<f64>>>| {
            let mut res = *num;
            for nb in nb {
                if nb.is_some() {
                    res += **nb.as_ref().unwrap();
                }
            }
            res % 255.0
        },
    );

    let nbs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    // let height = |x: usize, y: usize| {
    //     let dist = ((x as f64 - size.0 as f64 / 2.0) * (x as f64 - size.0 as f64 / 2.0)
    //         + (y as f64 - size.0 as f64 / 2.0) * (y as f64 - size.0 as f64 / 2.0))
    //         .sqrt();

    //     if dist < 300.0 {
    //         return 300.0;
    //     }
    //     50.0
    // };

    let height = |_x: usize, _y: usize| 1.0;

    let mut grid = Grid::new(size, op, 1, height, 2, 1, nbs);

    grid.calculate();

    grid.print();
}
