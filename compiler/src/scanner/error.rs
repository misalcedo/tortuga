//! Errors that may occur during lexical analysis.

use crate::{Lexeme, Location};
use std::fmt::{self, Display, Formatter};

/// An error that occurred during lexical analysis of a specific lexeme.
/// After an error is encountered, the scanner may continue to analyze the lexeme.
#[derive(Clone, Debug, PartialEq)]
pub struct LexicalError {
    lexeme: String,
    start: Location,
    kind: ErrorKind,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Encountered a {} error during lexical analysis on {}",
            self.kind, self.start
        )
    }
}

impl std::error::Error for LexicalError {}

/// The kind of lexical error that occurred.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Number,
    Identifier,
    Invalid,
}

impl LexicalError {
    /// Creates a new instance of a `LexicalError`.
    pub fn new<'a, L: Into<Lexeme<'a>>>(lexeme: L, kind: ErrorKind) -> Self {
        let lexeme = lexeme.into();

        LexicalError {
            lexeme: lexeme.as_str().to_string(),
            start: *lexeme.start(),
            kind,
        }
    }

    /// This `LexicalError`'s lexeme.
    pub fn as_str(&self) -> &str {
        &self.lexeme.as_str()
    }

    /// This `LexicalError`'s start [`Location`] in the input.
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// This `LexicalError`'s variant.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Number => f.write_str("NUMBER"),
            ErrorKind::Invalid => f.write_str("INVALID"),
            ErrorKind::Identifier => f.write_str("IDENTIFIER"),
        }
    }
}
