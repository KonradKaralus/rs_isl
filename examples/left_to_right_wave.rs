use core::f64;
use std::{
    cmp::max, path::PathBuf}
;

use rs_isl::*;

const DIM: (usize, usize) = (200, 100);

fn main() {
    let neighbours = vec![(-1, 0)];

    let op = |num: &f32, nb: Vec<Option<&f32>>| {
        if nb.first().unwrap().is_some() {
            let f = *nb[0].unwrap();
            if f > *num {
                return f;
            } else if f < *num {
                return f;
            } else {
                return *num;
            }
        }
        return max(*num as i32 - 3, 0) as f32;
    };

    let init = |x: usize, _y: usize| {
        if x < DIM.0 / 10 {
            let fac = x as f64 / (DIM.0 / 10) as f64 * f64::consts::FRAC_PI_2;

            return (250.0 - 250.0 * fac.sin()) as f32;
        }
        0.0
    };

    let params = IslParams::new(
        DIM,
        op,
        100,
        init,
        // DIM.0,
        150,
        75,
        neighbours,
        rs_isl::OutputType::VTK(PathBuf::from("raw")),
    );

    let _data = run_isl(params).unwrap();
}