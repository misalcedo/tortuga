use crate::Function;
use crate::Value;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Closure {
    function: Function,
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
            function: Function::default(),
            captures: Vec::default(),
        }
    }
}

impl Closure {
    pub fn new(function: Function, captures: Vec<Value>) -> Self {
        Closure { function, captures }
    }

    pub fn captures(&self) -> &[Value] {
        &self.captures[..]
    }
}
