use crate::translate::value::Value;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Capture {
    index: usize,
    local: bool,
    kind: Value,
}

impl Capture {
    pub fn new(index: usize, local: bool, kind: Value) -> Self {
        Capture { index, local, kind }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn local(&self) -> bool {
        self.local
    }

    pub fn kind(&self) -> Value {
        self.kind.clone()
    }
}
