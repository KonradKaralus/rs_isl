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

    return grid.calculate();
}
