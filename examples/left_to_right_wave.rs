use core::f64;
use std::{cmp::max, path::PathBuf};

use rs_isl::*;

const DIM: (usize, usize) = (100, 50);

fn main() {
    // we only access the left neighbour of every cell
    let neighbours = vec![(-1, 0)];

    // take neighbours value, if there is no neighbour decrease by 3
    let op = |num: &f32, nb: Vec<Option<&f32>>| {
        if nb.first().unwrap().is_some() {
            let f = *nb[0].unwrap();
            return f;
        }
        return max(*num as i32 - 3, 0) as f32;
    };

    // creates a sine shape at the left boundary of the domain
    let init = |x: usize, _y: usize| {
        if x < DIM.0 / 10 {
            let fac = x as f64 / (DIM.0 / 10) as f64 * f64::consts::FRAC_PI_2;
            return (250.0 - 250.0 * fac.sin()) as f32;
        }
        0.0
    };

    // create the simulation parameters
    let params = IslParams::new(
        DIM,
        op,
        // number of threads for simulation, the domain size must be divisible by this number
        100,
        init,
        // number of simulation steps
        DIM.0,
        // number of output steps
        DIM.0,
        neighbours,
        // path for writing vtk files
        PathBuf::from("raw"),
    );

    // run the simulation
    run_isl(params).unwrap();
}