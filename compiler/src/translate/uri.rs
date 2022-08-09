use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct ParseUriError(String);

impl Display for ParseUriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseUriError {}

impl From<&str> for ParseUriError {
    fn from(error: &str) -> Self {
        ParseUriError(error.to_string())
    }
}

impl From<String> for ParseUriError {
    fn from(error: String) -> Self {
        ParseUriError(error)
    }
}
