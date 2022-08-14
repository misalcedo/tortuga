use crate::Value;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Closure {
    function: usize,
    captures: Rc<Vec<Value>>,
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
            captures: Rc::new(vec![]),
        }
    }
}

impl Closure {
    pub fn new(function: usize, captures: Vec<Value>) -> Self {
        Closure {
            function,
            captures: Rc::new(captures),
        }
    }

    pub fn function(&self) -> usize {
        self.function
    }

    pub fn captures(&self) -> Rc<Vec<Value>> {
        Rc::clone(&self.captures)
    }
}
