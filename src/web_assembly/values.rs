#[derive(Copy, Clone)]
pub struct Limit {
    min: usize,
    max: Option<usize>,
}

impl Limit {
    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }
}
