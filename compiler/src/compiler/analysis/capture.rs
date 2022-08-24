use crate::compiler::analysis::value::Value;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Capture {
    parent: usize,
    offset: usize,
    local: bool,
    kind: Value,
}

impl Capture {
    pub fn new(parent: usize, offset: usize, local: bool, kind: Value) -> Self {
        Capture {
            parent,
            offset,
            local,
            kind,
        }
    }

    pub fn parent(&self) -> usize {
        self.parent
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn local(&self) -> bool {
        self.local
    }

    pub fn kind(&self) -> Value {
        self.kind.clone()
    }
}
