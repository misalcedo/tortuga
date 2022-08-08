use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function {
    start: usize,
    locals: u8,
    captures: Vec<bool>,
}

impl Function {
    pub fn new(start: usize, locals: u8, captures: Vec<bool>) -> Self {
        Function {
            start,
            locals,
            captures,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn locals(&self) -> u8 {
        self.locals
    }

    pub fn captures(&self) -> &[bool] {
        self.captures.as_slice()
    }
}
