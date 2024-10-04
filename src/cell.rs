use std::{fmt::Debug, mem::swap, sync::Arc};

use parking_lot::RwLock;

use crate::withcall::WithCall;

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
        F: Fn(&T, Vec<Option<&T>>) -> T,
        T: Debug,
    {
        let mut locks = vec![];
        let mut nbs = vec![];

        let own_lock = self.value.read();
        let own_ref = &*own_lock;

        self.neighbours.iter().for_each(|f| {
            if f.is_none() {
                locks.push(None);
            } else {
                let n = f.as_ref().unwrap();
                let n_val = n.read();
                locks.push(Option::from(n_val));
            }
        });

        for lock in &locks {
            let inner = lock.as_deref();
            nbs.push(inner);
        }

        self.next_val = op.run(own_ref, nbs);
    }

    pub fn write(&mut self) {
        let mut lt = self.value.write();
        swap(&mut *lt, &mut self.next_val);
    }
}
