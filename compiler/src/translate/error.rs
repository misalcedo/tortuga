use crate::translate::uri::ParseUriError;
use crate::translate::value::Value;
use std::fmt::{Display, Formatter};
use tortuga_executable::ParseNumberError;

#[derive(Clone, Debug, PartialEq)]
pub struct TranslationError {
    kind: ErrorKind,
}

impl From<ErrorKind> for TranslationError {
    fn from(kind: ErrorKind) -> Self {
        TranslationError { kind }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    InvalidNumber(ParseNumberError),
    InvalidUri(ParseUriError),
    OperandsMustBeNumbers(Value, Value),
    MissingChildren(usize, usize), // expected, actual
    TooManyLocals(usize),
    TooManyNumbers(usize),
    TooManyUris(usize),
    GroupTooLarge(usize),
    InvalidGroupSize(usize, usize), // expected, actual
    NoSuchFunction(usize),
    NoSuchNumber(usize),
    NoSuchUri(usize),
    NoSuchLocal(usize),
    NotCallable(Value),
    InvalidArguments(Vec<Value>, Vec<Value>), // parameters, arguments
    EmptyStack,
    EmptyContexts,
    ConditionWithoutAssignment,
    ExpectedEndOfBlock,
    ExpectedEndOfEquality,
}

impl Display for TranslationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TranslationError {}

impl From<ParseNumberError> for TranslationError {
    fn from(error: ParseNumberError) -> Self {
        TranslationError {
            kind: ErrorKind::InvalidNumber(error),
        }
    }
}

impl From<ParseUriError> for TranslationError {
    fn from(error: ParseUriError) -> Self {
        TranslationError {
            kind: ErrorKind::InvalidUri(error),
        }
    }
}
