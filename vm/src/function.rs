use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function {
    start: usize,
    locals: usize,
    captures: usize,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}/{}>", self.start, self.locals)
    }
}

impl Default for Function {
    fn default() -> Self {
        Function {
            start: 0,
            locals: 1,
            captures: 0,
        }
    }
}

impl Function {
    pub fn new(start: usize, locals: usize, captures: usize) -> Self {
        Function {
            start,
            locals,
            captures,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn locals(&self) -> usize {
        self.locals + 1
    }

    pub fn captures(&self) -> usize {
        self.captures
    }

    pub fn values(&self) -> usize {
        self.locals + 1 + self.captures
    }
}
