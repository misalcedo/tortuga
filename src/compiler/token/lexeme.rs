//! A lexeme is an excerpt of text from the source code to be compiled.
//! Lexeme's in the tortuga compiler also contain their `Location` information.

use crate::compile::Location;

/// A combination of a `Location` and an excerpt from the source code representing the lexeme.
#[derive(Debug, PartialEq)]
pub struct Lexeme<'source> {
    source: &'source str,
    start: Location,
}

impl<'source> Lexeme<'source> {
    /// Creates a new instance of a `Lexeme` with the given `Location` and lexeme.
    pub fn new(source: &'source str, start: Location) -> Self {
        Lexeme { source, start }
    }

    /// The start `Location` of this `Lexeme`.
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// The source text of this `Lexeme`.
    pub fn source(&self) -> &'source str {
        self.source
    }
}
