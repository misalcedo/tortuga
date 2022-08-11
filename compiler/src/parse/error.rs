//! Errors that may occur during lexical analysis.

use crate::scan::LexicalError;
use crate::Location;
use std::fmt::{self, Display, Formatter};

/// An error that occurred during parsing of the source code's syntax tree.
#[derive(Clone, Debug, PartialEq)]
pub struct SyntacticalError {
    message: String,
    incomplete: bool,
    start: Location,
}

impl Display for SyntacticalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error at ({}): {}", self.start, self.message)
    }
}

impl std::error::Error for SyntacticalError {}

impl From<LexicalError> for SyntacticalError {
    fn from(error: LexicalError) -> Self {
        SyntacticalError {
            message: format!("{}", &error),
            incomplete: false,
            start: *error.start(),
        }
    }
}

impl SyntacticalError {
    pub fn is_incomplete(&self) -> bool {
        self.incomplete
    }

    pub fn new(message: &str, start: Location) -> Self {
        SyntacticalError {
            message: message.to_string(),
            incomplete: false,
            start,
        }
    }

    pub fn incomplete(message: &str, start: Location) -> Self {
        SyntacticalError {
            message: message.to_string(),
            incomplete: true,
            start,
        }
    }
}
