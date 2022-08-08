use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function {
    start: usize,
    locals: usize,
    captures: Vec<bool>,
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
            captures: Vec::default(),
        }
    }
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

    pub fn locals(&self) -> usize {
        self.locals + 1
    }

    pub fn captures(&self) -> &[bool] {
        self.captures.as_slice()
    }

    pub fn values(&self) -> usize {
        self.locals + 1 + self.captures.len()
    }
}
