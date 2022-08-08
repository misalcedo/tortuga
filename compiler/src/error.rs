//! Errors that may occur during lexical analysis.

use crate::analysis::AnalyticalError;
use crate::parse::SyntacticalError;
use crate::scan::LexicalError;
use std::fmt::{self, Display, Formatter};

/// An error that occurred while compiling source code.
#[derive(Clone, Debug, PartialEq)]
pub struct CompilationError {
    message: String,
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CompilationError {}

impl CompilationError {
    pub fn new(message: &str) -> Self {
        CompilationError {
            message: message.to_string(),
        }
    }
}

impl From<LexicalError> for CompilationError {
    fn from(error: LexicalError) -> Self {
        CompilationError {
            message: format!("{}", &error),
        }
    }
}

impl From<SyntacticalError> for CompilationError {
    fn from(error: SyntacticalError) -> Self {
        CompilationError {
            message: format!("{}", &error),
        }
    }
}

impl From<AnalyticalError> for CompilationError {
    fn from(error: AnalyticalError) -> Self {
        CompilationError {
            message: format!("{}", &error),
        }
    }
}

impl From<&str> for CompilationError {
    fn from(error: &str) -> Self {
        CompilationError {
            message: error.to_string(),
        }
    }
}

impl From<String> for CompilationError {
    fn from(error: String) -> Self {
        CompilationError { message: error }
    }
}
