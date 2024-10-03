use parking_lot::RwLockReadGuard;
use rs_isl::{run_isl, IslParams};

fn main() {
    let size = (4, 2);
    let neighbours = vec![(-1, 0)];

    let op = |_num: RwLockReadGuard<f64>, nb: &Vec<Option<RwLockReadGuard<f64>>>| {
        if nb.first().unwrap().is_some() {
            let f = **nb[0].as_ref().unwrap();
            if f != 0.0 {
                return 1.0;
            }
        }
        0.0
    };
    let init = |x: usize, _y: usize| {
        if x == 0 {
            return 1.0;
        }
        0.0
    };

    let params = IslParams::new(
        size,
        op,
        1,
        init,
        4,
        4,
        neighbours,
        rs_isl::OutputType::String,
    );

    let data = run_isl(params);

    match data.unwrap() {
        rs_isl::IslOutput::RawData(vec) => println!("{:?}", vec),
        rs_isl::IslOutput::String(vec) => {
            for line in vec {
                println!("{}", line)
            }
        }
    }
}
