use crate::ParseNumberError;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct AnalysisError {
    kind: ErrorKind,
}

impl From<ErrorKind> for AnalysisError {
    fn from(kind: ErrorKind) -> Self {
        AnalysisError { kind }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    EmptyProgram,
    UnusedExpression,
    InvalidNumber(ParseNumberError),
    UninitializedLocal(usize),
    TooManyFunctions(usize),
    TooManyLocals(usize),
    TooManyCaptures(usize),
    TooManyNumbers(usize),
    TooManyUris(usize),
}

impl Display for AnalysisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AnalysisError {}

impl From<ParseNumberError> for AnalysisError {
    fn from(error: ParseNumberError) -> Self {
        AnalysisError {
            kind: ErrorKind::InvalidNumber(error),
        }
    }
}
