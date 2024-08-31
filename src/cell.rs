use std::sync::{atomic::{AtomicU64, Ordering}, Arc};

use crate::WithCall;

const ARRAY_REPEAT_VALUE: Option<Arc<AtomicF64>> = None;


#[derive(Clone)]
pub struct Cell {
    pub value: Arc<AtomicF64>,
    pub neighbours: [Option<Arc<AtomicF64>>; 4],
    pub next_val: f64,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            value: Arc::new(AtomicF64::new(1.0)),
            neighbours: [ARRAY_REPEAT_VALUE; 4],
            next_val: 0.0,
        }
    }

    pub fn run<F>(&mut self, op: &WithCall<F>)
    where
        F: Fn(f64, &[Option<f64>]) -> f64,
    {
        let nb_n: &[Option<f64>; 4] = &self
            .neighbours
            .iter()
            .map(|f| {
                if f.is_none() {
                    None
                } else {
                    let n = f.as_ref().unwrap();
                    let res = n.load(Ordering::Relaxed);
                    Option::from(res)
                }
            })
            .collect::<Vec<Option<f64>>>()
            .try_into()
            .unwrap();

        self.next_val = op.run(self.value.load(Ordering::Relaxed), nb_n);
    }

    pub fn write(&mut self) {
        self.value.store(self.next_val, Ordering::Relaxed);
    }
}

pub struct AtomicF64 {
    storage: AtomicU64,
}
impl AtomicF64 {
    pub fn new(value: f64) -> Self {
        let as_u64 = value.to_bits();
        Self {
            storage: AtomicU64::new(as_u64),
        }
    }
    pub fn store(&self, value: f64, ordering: Ordering) {
        let as_u64 = value.to_bits();
        self.storage.store(as_u64, ordering)
    }
    pub fn load(&self, ordering: Ordering) -> f64 {
        let as_u64 = self.storage.load(ordering);
        f64::from_bits(as_u64)
    }
}
