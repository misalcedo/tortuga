use crate::translate::value::Value;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use tortuga_executable::{Operation, ParseNumberError};

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
    OperandsMustBeNumbers(Value, Value),
    MissingChildren(RangeInclusive<usize>, usize), // expected, actual
    TooManyChildren(RangeInclusive<usize>, usize), // expected, actual
    TooManyLocals(usize),
    TooManyCaptures(usize),
    TooManyNumbers(usize),
    TooManyUris(usize),
    TooManyFunctions(usize),
    GroupTooLarge(usize),
    EmptyGroup,
    NoSuchFunction(usize),
    NoSuchNumber(usize),
    NoSuchUri(usize),
    NoSuchLocal(usize),
    NotCallable(Value),
    NotAssignable(Value),
    InvalidArguments(Value, Value), // parameters, arguments
    EmptyScopes,
    BlockOutsideFunction,
    ConditionOutsideFunction,
    ComparisonOutsideCondition(Operation),
    InvalidCondition(String),
    ExpectedKind(String, String), // expected, actual
    ReferenceSelfInInitializer(String),
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
