//! Errors that may occur during lexical analysis.

use crate::compiler::Location;
use crate::compiler::{Excerpt, LexicalError};
use std::fmt::{self, Display, Formatter};

/// An error that occurred during parsing of the source code's syntax tree.
#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxError {
    message: String,
    incomplete: bool,
    excerpt: Excerpt,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error at ({}): {}", self.excerpt, self.message)
    }
}

impl std::error::Error for SyntaxError {}

impl From<LexicalError> for SyntaxError {
    fn from(error: LexicalError) -> Self {
        SyntaxError {
            message: format!("{}", &error),
            incomplete: false,
            excerpt: *error.excerpt(),
        }
    }
}

impl SyntaxError {
    pub fn is_incomplete(&self) -> bool {
        self.incomplete
    }

    pub fn new(message: &str, excerpt: Excerpt) -> Self {
        SyntaxError {
            message: message.to_string(),
            incomplete: false,
            excerpt,
        }
    }

    pub fn incomplete(message: &str, start: Location) -> Self {
        SyntaxError {
            message: message.to_string(),
            incomplete: true,
            excerpt: Excerpt::from(start..),
        }
    }
}
