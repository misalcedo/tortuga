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
    ExpectedIdentifier(Value),
    ExpectedNumber(Value),
    ExpectedClosure(Value),
    UnsupportedOperation(usize),
    UnsupportedType(Value),
    UnsupportedTypes(Value, Value),
    InvalidOperand(usize, usize), // expected, actual
    NoSuchConstant(usize),
    NoSuchFunction(usize),
    TooManyLocals(usize),
    UndefinedLocal(usize, usize),      // requested, defined
    UndefinedCapture(usize, usize),    // requested, defined
    NotEnoughParameters(usize, usize), // expected, actual,
    FunctionMissingReturn(usize),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
