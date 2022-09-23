//! Errors that may occur during lexical scope analysis.

use crate::compiler::Excerpt;
use std::fmt::{self, Display, Formatter};

/// An error that occurred during parsing of the source code's syntax tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopeError {
    kind: ScopeErrorKind,
    excerpt: Excerpt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeErrorKind {}

impl Display for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexical scope error at ({}): {}",
            self.excerpt, self.message
        )
    }
}

impl std::error::Error for ScopeError {}

impl ScopeError {
    pub fn new(message: &str, excerpt: Excerpt) -> Self {
        ScopeError {
            message: message.to_string(),
            excerpt,
        }
    }
}
