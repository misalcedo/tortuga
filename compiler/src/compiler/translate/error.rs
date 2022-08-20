use crate::compiler::translate::value::Value;
use crate::{Operation, ParseNumberError};
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

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
    UnnecessaryParenthesis,
    NoSuchFunction(usize),
    NoSuchNumber(usize),
    NoSuchUri(usize),
    NoSuchLocal(usize),
    NotCallable(Value),
    InvalidArguments(Value, Value), // parameters, arguments
    EmptyScopes,
    EmptyBlock,
    BlockOutsideFunction,
    ConditionOutsideFunction,
    ComparisonOutsideCondition(Operation),
    InvalidCondition(String),
    ExpectedKind(String, String), // expected, actual
    ReferenceSelfInInitializer(String),
    PartiallyDeclaredFunction,
    FunctionAlreadyInitialized(usize),
    LocalInFunctionSignature(usize, usize), // function, parameter
    BlockNotTerminated,
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
