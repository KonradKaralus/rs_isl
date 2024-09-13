use std::{fmt::Debug, mem::swap, sync::Arc};

use parking_lot::{RwLock, RwLockReadGuard};

use crate::WithCall;

#[derive(Clone)]
pub struct Cell<T>
where
    T: Clone,
{
    pub value: Arc<RwLock<T>>,
    pub neighbours: Vec<Option<Arc<RwLock<T>>>>,
    pub next_val: T,
}

impl<T> Cell<T>
where
    T: Clone,
{
    pub fn run<F>(&mut self, op: &WithCall<F, T>)
    where
        F: Fn(RwLockReadGuard<T>, &Vec<Option<RwLockReadGuard<T>>>) -> T,
        T: Debug,
    {
        let nb_n = self
            .neighbours
            .iter()
            .map(|f| {
                if f.is_none() {
                    None
                } else {
                    let n = f.as_ref().unwrap();
                    Option::from(n.read())
                }
            })
            .collect::<Vec<Option<RwLockReadGuard<T>>>>();

        self.next_val = op.run(self.value.read(), &nb_n);
    }

    pub fn write(&mut self) {
        let mut lt = self.value.write();
        swap(&mut *lt, &mut self.next_val);
    }
}
