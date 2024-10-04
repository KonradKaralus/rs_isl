use parking_lot::RwLockReadGuard;

#[derive(Clone)]
pub struct WithCall<F, T>
where
    T: Clone,
{
    // TODO fix this, this does not need clone
    fp: F,
    _type: Option<T>, //this is a hacky workaround to make the below impl block work
}

impl<F, T> WithCall<F, T>
where
    F: Fn(&T, Vec<Option<&T>>) -> T,
    T: Clone,
{
    pub fn new(fp: F) -> Self {
        WithCall {
            fp,
            _type: Option::None,
        }
    }

    pub fn run(&self, a: &T, b: Vec<Option<&T>>) -> T {
        (self.fp)(a, b)
    }
}
