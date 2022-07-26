mod extractors;
mod operators;
mod wrappers;

use crate::{Closure, Identifier, Number};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value {
    Number(Number),
    Closure(Closure),
    Identifier(Identifier),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
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
