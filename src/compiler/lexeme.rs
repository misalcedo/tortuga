//! A lexeme is an excerpt of text from the source code to be compiled.
//! Lexeme's in the tortuga compiler are denoted by their start and end [`Location`]s.

use crate::compiler::Location;
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

/// An excerpt of the input and the [`Location`] of the start of the excerpt.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lexeme<'a> {
    start: Location,
    lexeme: Cow<'a, str>,
}

impl Display for Lexeme<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.start)
    }
}

impl From<String> for Lexeme<'_> {
    fn from(lexeme: String) -> Self {
        Lexeme {
            start: Location::default(),
            lexeme: lexeme.into(),
        }
    }
}

impl<'a> From<&'a str> for Lexeme<'a> {
    fn from(lexeme: &'a str) -> Self {
        Lexeme {
            start: Location::default(),
            lexeme: lexeme.into(),
        }
    }
}

impl<'a> Lexeme<'a> {
    /// Creates an owned clone of this [`Lexeme`].
    pub fn to_owned(&self) -> Lexeme<'static> {
        Lexeme {
            start: self.start,
            lexeme: Cow::Owned::<'static>(self.lexeme.to_string()),
        }
    }

    /// Creates a new instance of a `Lexeme` with the given start and end `Location`s.
    pub fn new<S: Into<Location>>(start: S, lexeme: &'a str) -> Self {
        Lexeme {
            start: start.into(),
            lexeme: lexeme.into(),
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
    pub fn as_str(&self) -> &str {
        &self.lexeme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexeme_owned_or_borrowed() {
        let input = "Hello, World!";
        let lexeme_str = Lexeme::from(input);
        let lexeme_string = Lexeme::from(input.to_string());

        assert_eq!(lexeme_str.as_str(), input);
        assert_eq!(lexeme_string.as_str(), input);
        assert_eq!(lexeme_str, lexeme_string);
    }

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
