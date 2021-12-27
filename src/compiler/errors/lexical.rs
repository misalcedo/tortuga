//! Errors that may occur during lexical analysis.

use crate::compiler::Lexeme;

/// An error that occurred during lexical analysis of a specific lexeme.
/// After an error is encountered, the scanner may continue to analyze the lexeme.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LexicalError {
    lexeme: Lexeme,
    kind: ErrorKind,
}

/// The kind of lexical error that occurred.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Number,
    Invalid,
}

impl LexicalError {
    /// Creates a new instance of a `LexicalError` with no cascading failures.
    pub fn new(lexeme: Lexeme, kind: ErrorKind) -> Self {
        LexicalError { lexeme, kind }
    }
}
