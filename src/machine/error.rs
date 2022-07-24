use crate::machine::Value;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeError {
    kind: ErrorKind,
}

impl From<ErrorKind> for RuntimeError {
    fn from(kind: ErrorKind) -> Self {
        RuntimeError { kind }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Crash,
    EmptyStack,
    ExpectedIdentifier(Value),
    UnsupportedOperation(u8),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
