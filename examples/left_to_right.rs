use isl::{run_isl, IslParams};
use parking_lot::RwLockReadGuard;

// fn main() {
//     let size = (500, 500);

//     let op = |num: RwLockReadGuard<f64>, nb: &Vec<Option<RwLockReadGuard<f64>>>| {
//         let mut res = *num;
//         for nb in nb {
//             if nb.is_some() {
//                 res += **nb.as_ref().unwrap();
//             }
//         }
//         res % 255.0
//     };

//     let neighbours = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

//     let height = |x: usize, y: usize| {
//         let dist = ((x as f64 - size.0 as f64 / 2.0) * (x as f64 - size.0 as f64 / 2.0)
//             + (y as f64 - size.0 as f64 / 2.0) * (y as f64 - size.0 as f64 / 2.0))
//             .sqrt();

//         if dist < 300.0 {
//             return 300.0;
//         }
//         50.0
//     };

//     run_isl(IslParams::new(size, op, 100, height, 10, 5, neighbours));
// }

fn main() {
    let size = (2, 2);

    let op = |num: RwLockReadGuard<f64>, nb: &Vec<Option<RwLockReadGuard<f64>>>| *num + 1.0;

    let neighbours = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let height = |x: usize, y: usize| 0.0;

    let o = run_isl(IslParams::new(
        size,
        op,
        1,
        height,
        10,
        10,
        neighbours,
        isl::OutputType::RawData,
    ));

    match o {
        isl::IslOutput::RawData(vec) => println!("{:?}", vec),
        isl::IslOutput::String(vec) => {
            for line in vec {
                println!("{}", line)
            }
        }
    }
}
