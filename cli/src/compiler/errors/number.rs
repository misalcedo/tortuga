//! Error parsing a numerical literal into a value.

use std::fmt::{Display, Formatter};

/// An error that occurred parsing a numerical literal into runtime value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseNumberError {
    lexeme: String,
}

impl Display for ParseNumberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse number \"{}\" to a runtime value.",
            self.lexeme
        )
    }
}

impl std::error::Error for ParseNumberError {}

impl ParseNumberError {
    /// The number literal that could not be parsed.
    pub fn lexeme(&self) -> &str {
        self.lexeme.as_str()
    }
}

impl From<&str> for ParseNumberError {
    fn from(lexeme: &str) -> Self {
        ParseNumberError {
            lexeme: lexeme.to_string(),
        }
    }
}
