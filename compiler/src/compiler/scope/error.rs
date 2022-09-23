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
pub enum ScopeErrorKind {
    ExitRootScope
}

impl Display for ScopeErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Display for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexical scope error at ({}): {}",
            self.excerpt, self.kind
        )
    }
}

impl std::error::Error for ScopeError {}

impl ScopeError {
    pub fn new(kind: ScopeErrorKind, excerpt: Excerpt) -> Self {
        ScopeError { kind, excerpt }
    }
}
