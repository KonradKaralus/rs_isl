use isl::{run_isl, IslParams};
use parking_lot::RwLockReadGuard;

fn main() {
    let size = (4, 2);

    let op = |num: RwLockReadGuard<f64>, nb: &Vec<Option<RwLockReadGuard<f64>>>| {
        if nb.first().unwrap().is_some() {
            let f = **nb[0].as_ref().unwrap();
            if f != 0.0 {
                return 1.0;
            }
        }
        0.0
    };

    let neighbours = vec![(-1, 0)];

    let height = |x: usize, y: usize| {
        if x == 0 {
            return 1.0;
        }
        0.0
    };

    let data = run_isl(IslParams::new(
        size,
        op,
        1,
        height,
        4,
        4,
        neighbours,
        isl::OutputType::String,
    ));

    match data {
        isl::IslOutput::RawData(vec) => println!("{:?}", vec),
        isl::IslOutput::String(vec) => {
            for line in vec {
                println!("{}", line)
            }
        }
    }
}
