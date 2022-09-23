//! Errors that may occur during lexical analysis.

use crate::compiler::parse::SyntaxError;
use crate::compiler::scan::LexicalError;
use crate::compiler::scope::ScopeError;
use std::fmt::{self, Display, Formatter};

/// An error that occurred while compiling source code.
#[derive(Clone, Debug, PartialEq)]
pub enum CompilationError {
    Lexical(LexicalError),
    Syntax(SyntaxError),
    Scope(ScopeError),
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CompilationError::Lexical(inner) => write!(f, "{}", inner),
            CompilationError::Syntax(inner) => write!(f, "{}", inner),
            CompilationError::Scope(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for CompilationError {}

impl From<LexicalError> for CompilationError {
    fn from(error: LexicalError) -> Self {
        CompilationError::Lexical(error)
    }
}

impl From<SyntaxError> for CompilationError {
    fn from(error: SyntaxError) -> Self {
        CompilationError::Syntax(error)
    }
}

impl From<ScopeError> for CompilationError {
    fn from(error: ScopeError) -> Self {
        CompilationError::Scope(error)
    }
}
