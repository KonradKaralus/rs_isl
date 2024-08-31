use grid::Grid;

mod cell;
mod grid;
mod writer;

const SIZE: usize = 2000;
const STEPS: usize = 50;
const OUT_STEPS: usize = 50;

#[derive(Clone)]
struct WithCall<F> {
    fp: F,
}

impl<F> WithCall<F>
where
    F: Fn(f64, &[Option<f64>]) -> f64,
{
    pub fn new(fp: F) -> Self {
        WithCall { fp }
    }

    pub fn run(&self, a: f64, b: &[Option<f64>]) -> f64 {
        (self.fp)(a, b)
    }
}

fn main() {
    let op = WithCall::new(|mut num: f64, nb: &[Option<f64>]| {
        let mut nb_n = nb.to_vec();

        nb_n.retain(|f| f.is_some());

        for nb in &nb_n {
            let nb = nb.unwrap();

            num += nb;
        }
        num % 255.0
    });

    let height = |x: usize, y: usize| {
        let dist = ((x as f64 - SIZE as f64 / 2.0) * (x as f64 - SIZE as f64 / 2.0)
            + (y as f64 - SIZE as f64 / 2.0) * (y as f64 - SIZE as f64 / 2.0))
            .sqrt();

        if dist < 300.0 {
            return 300.0;
        }
        50.0
    };

    let mut grid = Grid::new(SIZE, op, 100, height);

    grid.populate();
    grid.calculate();
    // grid.print();
}
