//! Scans a source file for valid characters.
//! The scanner produces a finite stream of characters, ignoring comments and blank space.

use std::iter::Peekable;
use std::str::Chars;
use crate::location::Location;

/// Scans a source text until completion.
/// Skips comments, new lines, and blank space.
#[derive(Clone, Debug)]
pub struct Scanner<'source> {
    source: &'source str,
    location: Location,
    cursor: Peekable<Chars<'source>>
}

impl<'a> PartialEq for Scanner<'a> {
    fn eq(&self, other: &Scanner<'a>) -> bool {
        self.source == other.source && self.location == other.location
    }
}

impl<'source> From<&'source str> for Scanner<'source> {
    fn from(source: &'source str) -> Self {
        Scanner {
            source,
            location: Location::default(),
            cursor: source.chars().peekable()
        }
    }
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` to scan the given source code starting at the given location.
    /// The location is not used to skip content in the source, but can be used to scan a chunked source across multiple scanners.
    pub fn continue_with(mut self, source: &'source str) -> Option<Self> {
        if self.cursor.peek().is_some() {
            return None;
        }

        Some(Scanner {
            source,
            location: self.location,
            cursor: source.chars().peekable()
        })
    }

    /// Peeks at the next character in the source.
    pub fn next(&mut self) -> Option<char> {
        self.cursor.next()
    }

    /// Peeks at the next character in the source.
    pub fn peek(&mut self) -> Option<char> {
        self.cursor.peek().map(|c| *c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue_with_consumed() {
        let mut scanner = Scanner::from("a");
        
        scanner.next();
        scanner = scanner.continue_with("bc").unwrap();

        assert_eq!(scanner.peek(), Some('b'));
    }

    #[test]
    fn continue_with_unfinished() {
        assert_eq!(Scanner::from("a").continue_with("bc"), None);
    }

    #[test]
    fn peek_empty() {
        assert_eq!(Scanner::from("").peek(), None);
    }

    #[test]
    fn peek_non_empty() {
        assert_eq!(Scanner::from("abc").peek(), Some('a'));
    }
}