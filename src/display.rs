use crate::compiler::Lexeme;
use std::fmt;
use std::fmt::Formatter;

/// A trait that defines how to get a [`Lexeme`] from types that can be displayed.
pub trait WithLexeme {
    /// The [`Lexeme`] of this object.
    fn lexeme(&self) -> Option<&Lexeme>;

    /// Create a [`LexemeString`] for this instance with the given source.
    fn as_display<'a>(&self, source: &'a str) -> LexemeString<'a> {
        match self.lexeme() {
            Some(&lexeme) => LexemeString { source, lexeme },
            None => LexemeString {
                source,
                lexeme: Lexeme::new(source, source),
            },
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
        f.write_str(self.lexeme.extract_from(self.source))
    }
}

impl WithLexeme for Lexeme {
    fn lexeme(&self) -> Option<&Lexeme> {
        self.into()
    }
}
