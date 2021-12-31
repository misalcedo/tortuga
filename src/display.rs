use crate::compiler::Lexeme;
use std::fmt;
use std::fmt::Formatter;

/// A trait that defines how to get a [`Lexeme`] from types that can be displayed.
pub trait WithLexeme {
    /// The [`Lexeme`] of this object.
    fn lexeme(&self) -> &Lexeme;

    /// Create a [`LexemeString`] for this instance with the given source.
    fn to_string_with<'a>(&self, source: &'a str) -> LexemeString<'a> {
        LexemeString {
            source,
            lexeme: *self.lexeme(),
        }
    }
}

/// A [`String`]-like type to display [`Lexeme`]s.
pub struct LexemeString<'a> {
    source: &'a str,
    lexeme: Lexeme,
}

impl<'a> fmt::Display for LexemeString<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.lexeme.lexeme().extract_from(self.source))
    }
}

impl WithLexeme for Lexeme {
    fn lexeme(&self) -> &Lexeme {
        self
    }
}
