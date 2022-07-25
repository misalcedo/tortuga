use crate::Value;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RuntimeError {
    kind: ErrorKind,
}

impl From<ErrorKind> for RuntimeError {
    fn from(kind: ErrorKind) -> Self {
        RuntimeError { kind }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorKind {
    EmptyStack,
    ExpectedIdentifier(Value),
    ExpectedNumber(Value),
    UnsupportedOperation(usize),
    UnsupportedTypes(Value, Value),
    InvalidOperand(usize, usize), // expected actual
    NoSuchConstant(usize),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
