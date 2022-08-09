//! Errors that may occur during lexical analysis.

use std::fmt::{self, Display, Formatter};

/// An error that occurred during the analysis of a program.
#[derive(Clone, Debug, PartialEq)]
pub struct AnalyticalError {
    message: String,
}

impl Display for AnalyticalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AnalyticalError {}

impl AnalyticalError {
    pub fn new(message: &str) -> Self {
        AnalyticalError {
            message: message.to_string(),
        }
    }
}
