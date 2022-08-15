use crate::translate::uri::ParseUriError;
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

impl From<&str> for TranslationError {
    fn from(kind: &str) -> Self {
        TranslationError {
            kind: ErrorKind::Unknown(kind.to_string()),
        }
    }
}

impl From<String> for TranslationError {
    fn from(kind: String) -> Self {
        TranslationError {
            kind: ErrorKind::Unknown(kind),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    InvalidNumber(ParseNumberError),
    InvalidUri(ParseUriError),
    MissingChildren(usize, usize), // expected, actual
    InvalidGroupingSize(usize),
    NoSuchLocal(usize),
    Unknown(String),
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
