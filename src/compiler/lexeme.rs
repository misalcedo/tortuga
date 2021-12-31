//! A lexeme is an excerpt of text from the source code to be compiled.
//! Lexeme's in the tortuga compiler are denoted by their start and end `Location`s.

use crate::compiler::Location;

/// A combination of a `Location` and a length in bytes.
/// Used to slice a source file to just the excerpt that is this `Lexeme`.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lexeme {
    start: Location,
    end: Location,
}

impl<L: Into<Location>> From<L> for Lexeme {
    fn from(end: L) -> Self {
        Lexeme {
            start: Location::default(),
            end: end.into(),
        }
    }
}

impl Lexeme {
    /// Creates a new instance of a `Lexeme` with the given start and end `Location`s.
    pub fn new<S: Into<Location>, E: Into<Location>>(start: S, end: E) -> Self {
        Lexeme {
            start: start.into(),
            end: end.into(),
        }
    }

    /// The start `Location` of this `Lexeme`.
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// The length in bytes of this `Lexeme`.
    pub fn len(&self) -> usize {
        self.end.offset() - self.start.offset()
    }

    /// Tests whether this `Lexeme` has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
        let start = Location::new(1, 8, 7);
        let end = Location::new(1, 13, 12);
        let lexeme = Lexeme::new(start, end);
        let input = "Hello, World!";

        assert_eq!(lexeme.len(), 5);
        assert_eq!(lexeme.is_empty(), false);
        assert_eq!(lexeme.extract_from(input), "World");
        assert_eq!(lexeme, Lexeme::new("Hello, ", "Hello, World"));
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
