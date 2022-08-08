use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;

#[derive(Clone, Debug, PartialEq)]
pub struct ParseNumberError(ParseFloatError);

impl Display for ParseNumberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseNumberError {}

impl From<ParseFloatError> for ParseNumberError {
    fn from(error: ParseFloatError) -> Self {
        ParseNumberError(error)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParseUriError {
    message: String,
}

impl Display for ParseUriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseUriError {}

impl From<&str> for ParseUriError {
    fn from(error: &str) -> Self {
        ParseUriError {
            message: error.to_string(),
        }
    }
}
