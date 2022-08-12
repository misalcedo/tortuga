#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Capture {
    index: usize,
    is_local: bool,
}

impl Capture {
    pub fn new(index: usize, is_local: bool) -> Self {
        Capture { index, is_local }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }
}
