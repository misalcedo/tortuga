use crate::translate::uri::ParseUriError;
use std::fmt::{Display, Formatter};
use tortuga_executable::ParseNumberError;

#[derive(Clone, Debug, PartialEq)]
pub struct TranslationError(String);

impl Display for TranslationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for TranslationError {}

impl From<&str> for TranslationError {
    fn from(error: &str) -> Self {
        TranslationError(error.to_string())
    }
}

impl From<String> for TranslationError {
    fn from(error: String) -> Self {
        TranslationError(error)
    }
}

impl From<ParseNumberError> for TranslationError {
    fn from(error: ParseNumberError) -> Self {
        TranslationError(format!("{}", error))
    }
}

impl From<ParseUriError> for TranslationError {
    fn from(error: ParseUriError) -> Self {
        TranslationError(format!("{}", error))
    }
}
