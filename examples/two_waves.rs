use core::f64;
use std::{cmp::max, path::PathBuf};

use rs_isl::*;

#[derive(Clone, Default)]
struct CellData {
    horizontal: i32,
    vertical: i32,
}

impl VtkOutput for CellData {
    fn cellvalue(&self) -> Vec<f32> {
        vec![self.horizontal as f32, self.vertical as f32]
    }

    fn value_names() -> Vec<String> {
        vec!["horizontal".into(), "vertical".into()]
    }
}

const DIM: (usize, usize) = (100, 100);

fn main() {
    // we only access the left neighbour of every cell
    let neighbours = vec![(-1, 0), (0, -1)];

    // take neighbours value, if there is no neighbour decrease by 3
    let op = |num: &CellData, nb: Vec<Option<&CellData>>| {
        let mut cd = CellData::default();
        if nb[0].is_some() {
            let f = nb[0].unwrap();
            cd.horizontal = f.horizontal;
        } else {
            cd.horizontal = max(num.horizontal - 3, 0);
        }
        if nb[1].is_some() {
            let f = nb[1].unwrap();
            cd.vertical = f.vertical;
        } else {
            cd.vertical = max(num.vertical - 3, 0);
        }
        cd
    };

    // creates a sine shape at the left boundary of the domain
    let init = |x: usize, y: usize| {
        let mut x_val = 0;
        let mut y_val = 0;
        if x < DIM.0 / 5 {
            let fac = x as f64 / (DIM.0 / 5) as f64 * f64::consts::FRAC_PI_2;
            x_val = (250.0 - 250.0 * fac.sin()) as i32;
        }
        if y < DIM.1 / 5 {
            let fac = y as f64 / (DIM.1 / 5) as f64 * f64::consts::FRAC_PI_2;
            y_val = (250.0 - 250.0 * fac.sin()) as i32;
        }
        // println!("ret for ({:?},{:?}): {:?},{:?}", x, y, x_val, y_val);
        CellData {
            horizontal: x_val,
            vertical: y_val,
        }
    };

    // create the simulation parameters
    let params = IslParams::new(
        DIM,
        op,
        // number of threads for simulation, the domain size must be divisible by this number
        1,
        init,
        // number of simulation steps
        100,
        // number of output steps
        100,
        neighbours,
        // path for writing vtk files
        PathBuf::from("raw"),
    );

    // run the simulation
    run_isl(params).unwrap();
}
