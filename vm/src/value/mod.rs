mod extractors;
mod operators;
mod wrappers;

use crate::{Closure, Identifier, Number};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Number(Number),
    Closure(Closure),
    Identifier(Identifier),
}

impl Default for Value {
    fn default() -> Self {
        Value::Number(Number::default())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Closure(c) => write!(f, "{}", c),
            Value::Identifier(i) => write!(f, "{}", i),
        }
    }
}
