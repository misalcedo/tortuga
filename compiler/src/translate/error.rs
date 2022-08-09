use std::fmt::{Display, Formatter};

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
