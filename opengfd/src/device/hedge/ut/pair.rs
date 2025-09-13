#[repr(C)]
pub struct Pair<F, S> {
    first: F,
    second: S
}

impl<F, S> Pair<F, S> {
    pub fn get_first(&self) -> &F {
        &self.first
    }
    pub fn get_second(&self) -> &S {
        &self.second
    }
}