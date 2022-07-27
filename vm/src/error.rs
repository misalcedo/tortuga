use crate::Value;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeError {
    kind: ErrorKind,
}

impl From<ErrorKind> for RuntimeError {
    fn from(kind: ErrorKind) -> Self {
        RuntimeError { kind }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    EmptyStack,
    EmptyCallFrames,
    CorruptedFrame,
    ExpectedIdentifier(Value),
    ExpectedNumber(Value),
    ExpectedClosure(Value),
    UnsupportedOperation(usize),
    UnsupportedTypes(Value, Value),
    InvalidOperand(usize, usize), // expected, actual
    NoSuchConstant(usize),
    IncorrectCall(usize, usize), // expected, actual,
    ReturnOutsideFunction,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
