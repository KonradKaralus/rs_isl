use core::f64;
use std::{cmp::max, path::PathBuf};

use rs_isl::*;

const DIM: (usize, usize) = (100, 50);

fn main() {
    // we only access the left neighbour of every cell
    let neighbours = vec![(-1, 0)];

    // This is the operation which is being run for every cell.
    // The cell's value and a vector of the values of its neighbours are provided.
    let op = |num: &f32, neighbours: Vec<Option<&f32>>| {
        let neighbour = neighbours.first().unwrap();
        // take neighbours value, if there is no neighbour decrease by 3

        if let Some(val) = neighbour {
            return **val;
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
