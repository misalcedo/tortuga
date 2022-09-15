//! Errors that may occur during lexical analysis.

use crate::compiler::{Excerpt, Location};
use std::fmt::{self, Display, Formatter};

/// An error that occurred during lexical analysis of a specific lexeme.
#[derive(Clone, Debug, PartialEq)]
pub struct LexicalError {
    message: String,
    excerpt: Excerpt,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Encountered an error during the lexical analysis on {}. Reason: {}",
            self.excerpt, self.message
        )
    }
}

impl std::error::Error for LexicalError {}

impl LexicalError {
    /// Creates a new instance of a [`LexicalError`].
    pub fn new(message: &str, start: Location, lexeme: &str) -> Self {
        LexicalError {
            message: message.to_string(),
            excerpt: Excerpt::from(start..(start + lexeme)),
        }
    }

    /// This [`LexicalError`]'s error message.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// This [`LexicalError`]'s variant.
    pub fn excerpt(&self) -> &Excerpt {
        &self.excerpt
    }
}
