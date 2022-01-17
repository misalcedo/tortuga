//! A lexeme is an excerpt of text from the source code to be compiled.
//! Lexeme's in the tortuga compiler are denoted by their start and end [`Location`]s.

use crate::compiler::Location;
use std::fmt::{self, Display, Formatter};

/// An excerpt of the input and the [`Location`] of the start of the excerpt.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lexeme<'a> {
    start: Location,
    lexeme: &'a str,
}

impl Display for Lexeme<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.start.fmt(f)
    }
}

impl<'a> From<&'a str> for Lexeme<'a> {
    fn from(lexeme: &'a str) -> Self {
        Lexeme {
            start: Location::default(),
            lexeme,
        }
    }
}

impl<'a> Lexeme<'a> {
    /// Creates a new instance of a `Lexeme` with the given start and end `Location`s.
    pub fn new<S: Into<Location>>(start: S, lexeme: &'a str) -> Self {
        Lexeme {
            start: start.into(),
            lexeme
        }
    }

    /// The start [`Location`] of this [`Lexeme`].
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// The length in bytes of this [`Lexeme`].
    pub fn len(&self) -> usize {
        self.lexeme.len()
    }

    /// Tests whether this [`Lexeme`] has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.lexeme.is_empty()
    }

    /// A [`str`] representing this [`Lexeme`] in the input.
    pub fn as_str(&self) -> &'a str {
        self.lexeme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_source() {
        let start = Location::new(1, 8, 7);
        let input = "Hello, World!";
        let lexeme = Lexeme::new(start, &input[start.offset()..]);

        assert_eq!(lexeme.len(), 6);
        assert!(!lexeme.is_empty());
        assert_eq!(lexeme, Lexeme::new("Hello, ", "World!"));
    }

    #[test]
    fn empty() {
        let lexeme = Lexeme::new(Location::default(), "");

        assert_eq!(lexeme.len(), 0);
        assert!(lexeme.is_empty());
    }
}
