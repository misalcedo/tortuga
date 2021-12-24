//! A lexeme is an excerpt of text from the source code to be compiled.
//! Lexeme's in the tortuga compiler also contain their `Location` information.

use crate::compiler::Location;

/// A combination of a `Location` and a length in bytes.
/// Used to slice a source file to just the excerpt that is this `Lexeme`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lexeme {
    start: Location,
    length: usize,
}

impl Lexeme {
    /// Creates a new instance of a `Lexeme` with the given `Location` and length in bytes.
    pub fn new(start: Location, length: usize) -> Self {
        Lexeme { start, length }
    }

    /// The start `Location` of this `Lexeme`.
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// The length in bytes of this `Lexeme`.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Tests whether this `Lexeme` has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Extracts this `Lexeme` from the given input.
    ///
    /// ## Panic
    /// Panics when the given input is shorter than the offset plus length of this `Lexeme`.
    pub fn extract_from<'a>(&self, input: &'a str) -> &'a str {
        let offset = self.start.offset();
        let end = offset + self.length;

        &input[offset..end]
    }
}

/// A source for lexemes.
pub trait LexemeSource {
    fn lexeme(&self, lexeme: &Lexeme) -> &Self;
}

impl LexemeSource for str {
    fn lexeme(&self, lexeme: &Lexeme) -> &Self {
        lexeme.extract_from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_source() {
        let lexeme = Lexeme::new(Location::new(1, 1, 7), 5);
        let input = "Hello, World!";

        assert_eq!(lexeme.extract_from(input), "World");
        assert_eq!(input.lexeme(&lexeme), "World");
    }
}
