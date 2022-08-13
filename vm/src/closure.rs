use crate::Function;
use crate::Value;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Closure {
    function: usize,
    captures: Vec<Value>,
}

impl Display for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.function)
    }
}

impl Default for Closure {
    fn default() -> Self {
        Closure {
            function: 0,
            captures: Vec::default(),
        }
    }
}

impl From<Closure> for Vec<Value> {
    fn from(closure: Closure) -> Self {
        closure.captures
    }
}

impl Closure {
    pub fn new(function: usize, captures: Vec<Value>) -> Self {
        Closure { function, captures }
    }

    pub fn function(&self) -> usize {
        self.function
    }

    pub fn captures(&self) -> &[Value] {
        &self.captures[..]
    }
}
