//! Errors that may occur during lexical analysis.

use crate::Location;
use std::fmt::{self, Display, Formatter};

/// An error that occurred during lexical analysis of a specific lexeme.
/// After an error is encountered, the scanner may continue to analyze the lexeme.
#[derive(Clone, Debug, PartialEq)]
pub struct SyntacticalError {
    message: String,
    start: Location,
}

impl Display for SyntacticalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for SyntacticalError {}
