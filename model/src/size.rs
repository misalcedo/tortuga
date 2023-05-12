use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hint(usize, Option<usize>);

impl Hint {
    pub fn new(lower: usize, upper: Option<usize>) -> Self {
        Hint(lower, upper)
    }

    pub fn exact(length: usize) -> Self {
        Hint(length, Some(length))
    }

    pub fn lower(&self) -> usize {
        self.0
    }

    pub fn upper(&self) -> Option<usize> {
        self.1
    }
}
