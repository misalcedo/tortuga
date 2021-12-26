//! Errors that may occur during lexical analysis.

use crate::compiler::Lexeme;

/// An error that occurred during lexical analysis of a specific lexeme.
/// After an error is encountered, the scanner may continue to analyze the lexeme.
/// If cascading failures occur, those can be associated with this error as well.
pub struct LexicalError {
    lexeme: Lexeme,
    kind: ErrorKind,
    cascade: Vec<ErrorKind>,
}

/// The kind of lexical error that occurred.
pub enum ErrorKind {
    Number,
    Identifier,
    Invalid,
}

impl LexicalError {
    /// Creates a new instance of a `LexicalError` with no cascading failures.
    pub fn new(lexeme: Lexeme, kind: ErrorKind) -> Self {
        LexicalError {
            lexeme,
            kind,
            cascade: Vec::new(),
        }
    }

    /// Creates a new instance of a `LexicalError` with cascading failures.
    pub fn new_cascading(lexeme: Lexeme, kind: ErrorKind, cascade: Vec<ErrorKind>) -> Self {
        LexicalError {
            lexeme,
            kind,
            cascade,
        }
    }
}
