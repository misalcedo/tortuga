//! A lexeme is an excerpt of text from the source code to be compiled.
//! Lexeme's in the tortuga compiler also contain their `Location` information.

use crate::compiler::Location;

/// A combination of a `Location` and a length in bytes.
/// Used to slice a source file to just the excerpt that is this `Lexeme`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lexeme {
    start: Location,
    end: Location,
}

impl Lexeme {
    /// Creates a new instance of a `Lexeme` with the given start and end `Location`s.
    pub fn new(start: Location, end: Location) -> Self {
        Lexeme { start, end }
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
        let start = self.start.offset();
        let end = self.end.offset();

        &input[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_source() {
        let start = Location::new(1, 1, 7);
        let end = Location::new(1, 1, 12);
        let lexeme = Lexeme::new(start, end);
        let input = "Hello, World!";

        assert_eq!(lexeme.len(), 12);
        assert_eq!(lexeme.is_empty(), false);
        assert_eq!(lexeme.extract_from(input), "World");
    }

    #[test]
    fn empty() {
        let lexeme = Lexeme::new(Location::default(), Location::default());
        let input = "Hello, World!";

        assert_eq!(lexeme.len(), 0);
        assert_eq!(lexeme.is_empty(), true);
        assert_eq!(lexeme.extract_from(input), "");
    }
}
