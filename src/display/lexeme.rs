//! Utility for extracting and displaying [`Lexeme`]s.

use crate::compiler::Lexeme;
use std::fmt;

/// A trait that defines how to get a [`Lexeme`] from types that can be displayed.
pub trait WithLexeme {
    /// The [`Lexeme`] of this object.
    fn lexeme(&self) -> &Lexeme;

    /// Create a [`LexemeString`] for this instance with the given source.
    fn as_display<'a>(&self, source: &'a str) -> LexemeString<'a> {
        LexemeString {
            source,
            lexeme: *self.lexeme(),
        }
    }
}

/// A [`String`]-like type to display [`Lexeme`]s.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LexemeString<'a> {
    source: &'a str,
    lexeme: Lexeme,
}

impl<'a> fmt::Display for LexemeString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.lexeme.extract_from(self.source))
    }
}

impl WithLexeme for Lexeme {
    fn lexeme(&self) -> &Lexeme {
        self
    }
}
