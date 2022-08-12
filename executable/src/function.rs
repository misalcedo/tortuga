use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Function {
    start: usize,
    locals: usize,
    captures: Vec<bool>,
}

impl Function {
    pub fn new(start: usize, locals: usize, captures: Vec<bool>) -> Self {
        Function {
            start,
            locals,
            captures,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn set_locals(&mut self, locals: usize) {
        self.locals = locals;
    }

    pub fn locals(&self) -> usize {
        self.locals
    }

    pub fn captures(&self) -> &[bool] {
        self.captures.as_slice()
    }

    pub fn values(&self) -> usize {
        self.locals + 1 + self.captures.len()
    }
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
            locals: 0,
            captures: Vec::default(),
        }
    }
}
