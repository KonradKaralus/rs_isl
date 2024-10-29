// ! ## rs_isl
// ! Implementation of Iterative Stencil Loops
// !
// ! Runs a simulation over a 2-dimensional array
// ! specified by the given parameters.
// !
// ! ## Features:
// ! - generic array elements
// ! - parallelization using threads
// ! - custom definition of neighbouring elements
// !
// ! For further information on ISLs see: https://wikipedia.org/wiki/Iterative_Stencil_Loops
// !
// ! ## Usage
// !
// ! An example which creates a wave-like motion from left to right through the grid.
// !
// ! ```rust, no_run
// ! use core::f64;
// ! use std::{cmp::max, path::PathBuf};
// !
// ! use rs_isl::*;
// !
// !
// ! fn main() {
// !     // create a domain with a size of 200 by 100
// !     let dim = (200, 100)
// !     // we only access the left neighbour of every cell
// !     let neighbours = vec![(-1, 0)];
// !
// !     // take neighbours value, if there is no neighbour decrease by 3
// !     let op = |num: &f32, nb: Vec<Option<&f32>>| {
// !         if nb.first().unwrap().is_some() {
// !             let f = *nb[0].unwrap();
// !             return f;
// !         }
// !         return max(*num as i32 - 3, 0) as f32;
// !     };
// !
// !     // creates a sine shape at the left boundary of the domain
// !     let init = |x: usize, _y: usize| {
// !         if x < DIM.0 / 10 {
// !             let fac = x as f64 / (DIM.0 / 10) as f64 * f64::consts::FRAC_PI_2;
// !             return (250.0 - 250.0 * fac.sin()) as f32;
// !         }
// !         0.0
// !     };
// !
// !     // create the simulation parameters
// !     let params = IslParams::new(
// !         dim,
// !         op,
// !         // number of threads for simulation, the domain size must be divisible by this number
// !         10,
// !         init,
// !         // number of simulation steps
// !         200,
// !         // number of output steps
// !         100,
// !         neighbours,
// !         // path for writing vtk files
// !         PathBuf::from("raw"),
// !     );
// !
// !     // run the simulation
// !     run_isl(params).unwrap();
// ! }
// ! ```

use grid::{Grid, InvalidThreadNumber};
use std::path::PathBuf;
use withcall::WithCall;

mod cell;
mod grid;
mod vtk_writer;
mod withcall;

/// Trait for defining the output of every cell.
///
/// Implement for your Data Type to write your data into the output file
///
/// # Example
/// ```rust, no_run
/// struct Point {
///     x: u32,
///     y: u32,
/// }
///
/// impl VtkOutput for Point {
///     fn value_names() -> Vec<String> {
///         vec!["x_coord".into(), "y_coord".into()]
///     }
///     fn cellvalue(&self) -> Vec<f32> {
///         vec![self.x as f32, self.y as f32]
///     }
/// }
/// ```
pub trait VtkOutput {
    /// Names for the DataArrays created with the values of every cell
    fn value_names() -> Vec<String>;

    /// Values for every cell, these will be written to the DataArrays, identified by their name
    fn cellvalue(&self) -> Vec<f32>;
}

impl<T> VtkOutput for T
where
    T: Into<f32> + Clone,
{
    fn cellvalue(&self) -> Vec<f32> {
        vec![(*self).clone().into()]
    }

    fn value_names() -> Vec<String> {
        vec!["val:".to_string()]
    }
}

pub struct IslParams<T, F, H>
where
    T: Clone + Sync + Send,
    F: Fn(&T, Vec<Option<&T>>) -> T + Clone + Send + Copy,
    H: Fn(usize, usize) -> T,
{
    pub dimension: (usize, usize),
    pub op: F,
    pub runners: usize,
    pub height: H,
    pub steps: usize,
    pub output_steps: usize,
    pub neighbours: Vec<(i8, i8)>,
    pub output_path: PathBuf,
}

impl<T, F, H> IslParams<T, F, H>
where
    T: Clone + Sync + Send,
    F: Fn(&T, Vec<Option<&T>>) -> T + Clone + Send + Copy,
    H: Fn(usize, usize) -> T,
{
    /// Set parameters for running an ISL
    ///
    /// * `dimension` - The size of the 2d-array, (x,y).
    /// * `operation` - The operation calculating each cell's new value.
    /// * `runners` - Number of threads used for running the ISL.
    /// * `init` - The closure, from which each cell's initial value will be calculated.
    /// * `steps` - Number of iterations.
    /// * `output_steps` - Number of output files returned.
    /// * `neighbours` - Definition of each cells neighbours, represented by their offsets.
    /// * `output_type` - Whether to return raw data or formatted strings.
    ///
    pub fn new(
        dimension: (usize, usize),
        operation: F,
        runners: usize,
        init: H,
        steps: usize,
        output_steps: usize,
        neighbours: Vec<(i8, i8)>,
        output_path: PathBuf,
    ) -> Self {
        Self {
            dimension,
            op: operation,
            runners,
            height: init,
            steps,
            output_steps,
            neighbours,
            output_path,
        }
    }
}
/// Runs the ISL and returns the output data
///
/// For more information see crate level documentation and [IslParams::new]
///
/// # Errors
///
/// If the given array size (x*y) is not divisible by the number of runners, an error will be returned.
pub fn run_isl<T, F, H>(options: IslParams<T, F, H>) -> Result<(), InvalidThreadNumber>
where
    T: Clone + Sync + Send + VtkOutput,
    F: Fn(&T, Vec<Option<&T>>) -> T + Clone + Send + Copy,
    H: Fn(usize, usize) -> T,
{
    let op = WithCall::new(options.op);

    let r_grid = Grid::new(
        options.dimension,
        op,
        options.runners,
        options.height,
        options.steps,
        options.output_steps,
        options.neighbours,
        options.output_path,
    );

    match r_grid {
        Ok(mut grid) => Ok(grid.calculate()),
        Err(_) => Err(InvalidThreadNumber()),
    }
}
