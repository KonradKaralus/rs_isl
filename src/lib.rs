//! ## rs_isl
//! Implementation of Iterative Stencil Loops
//!
//! Runs a simulation over a 2-dimensional array
//! specified by the given parameters.
//!
//! ### Features:
//! - generic array elements
//! - parallelization using threads
//! - custom definition of neighbouring elements
//!
//! For further information on ISLs see: https://wikipedia.org/wiki/Iterative_Stencil_Loops
//!
//! ## Usage
//!  
//! An example which creates a wave-like motion from left to right through the grid.
//!
//! ```rust, no_run
//! use rs_isl::IslParams;
//! use rs_isl::run_isl;
//!
//! // grid with a size of 4x2, where cells only access their left neighbour
//! let size = (4, 2);
//! let neighbours = vec![(-1, 0)];
//!
//! // closure that calculates the new value based on the cell's own value and it's neighbours
//! let op = |_num: &f64, nb: Vec<Option<&f64>>| {
//!     if nb.first().unwrap().is_some() {
//!         let f = nb[0].unwrap();
//!         // if the cell's neighbour has the value 1.0, we take that, otherwise we return 0.0
//!         if *f != 0.0 {
//!             return 1.0;
//!         }
//!     }
//!     0.0
//! };
//!
//! // closure that determines each cell's initial value based on it's position
//! let init = |x: usize, _y: usize| {
//!     // return 1.0 if the cell is located on the left boundary of the grid
//!     if x == 0 {
//!         return 1.0;
//!     }
//!     0.0
//! };
//!
//! // create parameters
//! let params = IslParams::new(
//!     size,
//!     op,
//!     // number of threads, the grids size (x*y) must be divisible by this value
//!     1,
//!     init,
//!     // number of steps for which the simulation will be run
//!     4,
//!     // number of output steps, these will be evenly distributed through the simulation
//!     4,
//!     neighbours,
//!     // type of returned data
//!     rs_isl::OutputType::String,
//! );
//!
//! // run ISL
//! let data = run_isl(params);
//!
//! // extract the data
//! match data.unwrap() {
//!     rs_isl::IslOutput::RawData(vec) => println!("{:?}", vec),
//!     rs_isl::IslOutput::String(vec) => {
//!         for line in vec {
//!             println!("{}", line)
//!         }
//!     }
//! }

use std::{fmt::Debug, path::PathBuf};

use grid::{Grid, InvalidThreadNumber};
use withcall::WithCall;

mod cell;
mod grid;
mod withcall;
mod vtk_writer;

/// Defines the output type
pub enum OutputType {
    /// Returns a vec with the raw values of the entire grid at each output step.
    RawData,
    /// Returns a vec with values gathered by calling [std::fmt::Debug] on each cell.
    ///
    /// The values will be separated by commas and each x line of the 2d array will be written to a new line.
    ///
    /// This can be useful if you want to serialize your data or if you don't need all of your cell data in the output.
    ///
    /// Implement [std::fmt::Debug] accordingly.
    String,
    VTK(PathBuf)
}

/// See [OutputType] for information on the different types of output.
#[derive(PartialEq)]
pub enum IslOutput<T> {
    RawData(Vec<Vec<Vec<T>>>),
    String(Vec<String>),
    VTK
}

pub struct IslParams<T, F, H>
where
    T: Clone + Debug + std::marker::Sync + std::marker::Send,
    F: Fn(&T, Vec<Option<&T>>) -> T + Clone + std::marker::Send + Copy,
    H: Fn(usize, usize) -> T,
{
    pub dimension: (usize, usize),
    pub op: F,
    pub runners: usize,
    pub height: H,
    pub steps: usize,
    pub output_steps: usize,
    pub neighbours: Vec<(i8, i8)>,
    pub output_type: OutputType,
}

impl<T, F, H> IslParams<T, F, H>
where
    T: Clone + Debug + std::marker::Sync + std::marker::Send,
    F: Fn(&T, Vec<Option<&T>>) -> T + Clone + std::marker::Send + Copy,
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
        output_type: OutputType,
    ) -> Self {
        Self {
            dimension,
            op: operation,
            runners,
            height: init,
            steps,
            output_steps,
            neighbours,
            output_type,
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
pub fn run_isl<T, F, H>(options: IslParams<T, F, H>) -> Result<IslOutput<T>, InvalidThreadNumber>
where
    T: Clone + Debug + std::marker::Sync + std::marker::Send + Into<f32>,
    F: Fn(&T, Vec<Option<&T>>) -> T + Clone + std::marker::Send + Copy,
    H: Fn(usize, usize) -> T,
    // f32: From<T>
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
        options.output_type,
    );

    match r_grid {
        Ok(mut grid) => return Ok(grid.calculate()),
        Err(_) => return Err(InvalidThreadNumber()),
    }
}
