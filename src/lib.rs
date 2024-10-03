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
//! // grid with a size of 4x2, where cells only access their left neighbour
//! let size = (4, 2);
//! let neighbours = vec![(-1, 0)];
//!
//! // closure that calculates the new value based on the cell's own value and it's neighbours
//! let op = |_num: RwLockReadGuard<f64>, nb: &Vec<Option<RwLockReadGuard<f64>>>| {
//!     if nb.first().unwrap().is_some() {
//!       let f = **nb[0].as_ref().unwrap();
//!         // if the cell's neighbours has the value 1.0, we take that, otherwise we return 0.0
//!         if f != 0.0 {
//!             return 1.0;
//!         }
//!     }
//!     0.0
//! };
//! 
//! // closure that determines each cell's initial value based on it's position
//! let init = |x: usize, y: usize| {
//!     // return 1.0 if the cell is located on the left boundary of the grid
//!     if x == 0 {
//!         return 1.0;
//!     }
//!     0.0
//! };
//!
//! 
//! // create parameters and run the simulation
//! let data = run_isl(IslParams::new(
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
//! ));
//! ```
//!

use std::fmt::Debug;

use grid::Grid;
use parking_lot::RwLockReadGuard;
use withcall::WithCall;

mod cell;
mod grid;
mod withcall;

pub enum OutputType {
    RawData,
    String,
}

#[derive(PartialEq)]
pub enum IslOutput<T> {
    RawData(Vec<Vec<Vec<T>>>),
    String(Vec<String>),
}

pub struct IslParams<T, F, H>
where
    T: Clone + Default + Debug + std::marker::Sync + std::marker::Send,
    F: Fn(RwLockReadGuard<T>, &Vec<Option<RwLockReadGuard<T>>>) -> T
        + Clone
        + std::marker::Send
        + Copy,
    H: Fn(usize, usize) -> T,
{
    dimension: (usize, usize),
    op: F,
    runners: usize,
    height: H,
    steps: usize,
    output_steps: usize,
    neighbours: Vec<(i8, i8)>,
    output_type: OutputType,
}

impl<T, F, H> IslParams<T, F, H>
where
    T: Clone + Default + Debug + std::marker::Sync + std::marker::Send,
    F: Fn(RwLockReadGuard<T>, &Vec<Option<RwLockReadGuard<T>>>) -> T
        + Clone
        + std::marker::Send
        + Copy,
    H: Fn(usize, usize) -> T,
{
    pub fn new(
        dimension: (usize, usize),
        op: F,
        runners: usize,
        height: H,
        steps: usize,
        output_steps: usize,
        neighbours: Vec<(i8, i8)>,
        output_type: OutputType,
    ) -> Self {
        Self {
            dimension,
            op,
            runners,
            height,
            steps,
            output_steps,
            neighbours,
            output_type,
        }
    }
}
///
///
///
///
///
///
///
pub fn run_isl<T, F, H>(options: IslParams<T, F, H>) -> IslOutput<T>
where
    T: Clone + Default + Debug + std::marker::Sync + std::marker::Send,
    F: Fn(RwLockReadGuard<T>, &Vec<Option<RwLockReadGuard<T>>>) -> T
        + Clone
        + std::marker::Send
        + Copy,
    H: Fn(usize, usize) -> T,
{
    let op = WithCall::new(options.op);

    let mut grid = Grid::new(
        options.dimension,
        op,
        options.runners,
        options.height,
        options.steps,
        options.output_steps,
        options.neighbours,
        options.output_type,
    );

    grid.calculate()
}
