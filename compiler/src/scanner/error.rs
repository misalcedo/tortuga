//! Errors that may occur during lexical analysis.

use crate::Location;
use std::fmt::{self, Display, Formatter};

/// An error that occurred during lexical analysis of a specific lexeme.
#[derive(Clone, Debug, PartialEq)]
pub struct LexicalError {
    message: String,
    lexeme: String,
    start: Location,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Encountered an error during the lexical analysis of {:?} on {}. Reason: {}",
            self.lexeme, self.start, self.message
        )
    }
}

impl std::error::Error for LexicalError {}

impl LexicalError {
    /// Creates a new instance of a [`LexicalError`].
    pub fn new(message: &str, start: Location, lexeme: &str) -> Self {
        LexicalError {
            message: message.to_string(),
            lexeme: lexeme.to_string(),
            start,
        }
    }

    /// This [`LexicalError`]'s error message.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// This [`LexicalError`]'s start [`Location`] in the input.
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// This [`LexicalError`]'s variant.
    pub fn lexeme(&self) -> &str {
        self.lexeme.as_str()
    }
}
