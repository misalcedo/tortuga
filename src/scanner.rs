//! Scans a source file for valid characters.
//! The scanner produces a finite stream of characters, ignoring comments and blank space.

use std::iter::Peekable;
use std::str::Chars;
use crate::location::{Location, LexemeSource};

/// Scans a source text until completion.
/// Skips comments, new lines, and blank space.
#[derive(Clone, Debug)]
pub struct Scanner<'source> {
    source: &'source str,
    start: Location,
    end: Location,
    cursor: Peekable<Chars<'source>>
}

impl<'a> PartialEq for Scanner<'a> {
    fn eq(&self, other: &Scanner<'a>) -> bool {
        self.source == other.source && self.start == other.start && self.end == other.end
    }
}

impl<'source> From<&'source str> for Scanner<'source> {
    fn from(source: &'source str) -> Self {
        Scanner {
            source,
            start: Location::default(),
            end: Location::default(),
            cursor: source.chars().peekable()
        }
    }
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` to scan the given source code starting at the given location.
    /// If the current `Scanner` has not fully scanned the source, returns None. 
    /// The location is not used to skip content in the source, but can be used to scan a chunked source across multiple scanners.
    pub fn continue_with(mut self, source: &'source str) -> Option<Self> {
        if self.cursor.peek().is_some() {
            return None;
        }

        Some(Scanner {
            source,
            start: self.end.continuation(),
            end: self.end.continuation(),
            cursor: source.chars().peekable()
        })
    }

    /// Skips comments until the end of the current line.
    fn skip_comment(&mut self) {
        while let Some(c) = self.cursor.next_if(|c| c != &'\n') {
            self.end.add_column(c);
        }
    }

    fn step_forward(&mut self) {
        self.start = self.end;
    }

    /// Skips any tokens are not meant to be part of a lexeme.
    fn skip(&mut self) {
        loop {
            match self.cursor.next_if(|c| c == &'\r' || c == &'\n' || c == &'\t' || c == &' ' || c == &';') {
                Some(';') => self.skip_comment(),
                Some('\n') => self.end.next_line(),
                Some(c @ '\r') => self.end.add_offset(c),
                Some(c @ ('\t' | ' ')) => self.end.add_column(c),
                _ => break
            };

            self.step_forward()
        }
    }

    /// Gets the next character in the source.
    /// Skips comments, blank space, and new lines.
    pub fn next(&mut self) -> Option<char> {
        self.skip();

        let c = self.cursor.next()?;
        
        self.end.add_column(c);
        
        Some(c)
    }

    /// Returns the next character only if the next one equals the expected value.
    pub fn next_if_eq(&mut self, expected: char) -> Option<char> {
        self.skip();
        
        self.cursor.next_if_eq(&expected)
    }

    /// Returns the next character only if the next one matches the given predicate.
    pub fn next_if(&mut self, predicate: impl FnOnce(&char) -> bool) -> Option<char> {
        self.skip();
        
        self.cursor.next_if(predicate)
    }

    /// Peeks at the next character in the source.
    pub fn peek(&mut self) -> Option<char> {
        self.cursor.peek().map(|c| *c)
    }

    /// Gets the lexeme starting at this scanner's location (inclusive) until the given end location (exclusive).
    pub fn lexeme(&mut self) -> &'source str {
        let substring = self.source.lexeme(&self.start, &self.end);
        
        self.step_forward();
        
        substring
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

        assert_eq!(scanner.next(), Some('b'));
        assert_eq!(scanner.lexeme(), "b");
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

    #[test]
    fn next_if_eq_when_equal() {
        assert_eq!(Scanner::from("abc").next_if_eq('a'), Some('a'));
    }

    #[test]
    fn next_if_eq_when_not_equal() {
        assert_eq!(Scanner::from("abc").next_if_eq('b'), None);
    }

    #[test]
    fn next_if_eq_when_empty() {
        assert_eq!(Scanner::from("").next_if_eq('a'), None);
    }

    #[test]
    fn next_if_when_true() {
        assert_eq!(Scanner::from("abc").next_if(|c| c.is_ascii_alphabetic()), Some('a'));
    }

    #[test]
    fn next_if_when_false() {
        assert_eq!(Scanner::from("abc").next_if(|c| c.is_ascii_digit()), None);
    }

    #[test]
    fn next_if_when_empty() {
        assert_eq!(Scanner::from("").next_if(|_| true), None);
    }

    #[test]
    fn next_normal() {
        assert_eq!(Scanner::from("abc").next(), Some('a'));
    }

    #[test]
    fn next_when_skipping() {
        assert_eq!(Scanner::from("\t abc").next(), Some('a'));
    }

    #[test]
    fn next_when_skipping_until_end() {
        assert_eq!(Scanner::from("; abc").next(), None);
    }

    #[test]
    fn next_when_empty() {
        assert_eq!(Scanner::from("").next(), None);
    }

    #[test]
    fn lexeme() {
        let mut scanner = Scanner::from("abc");

        scanner.next();
        scanner.next();

        let first = scanner.lexeme();

        scanner.next();

        let second = scanner.lexeme();
        let third = scanner.lexeme();

        assert_eq!(first, "ab");
        assert_eq!(second, "c");
        assert_eq!(third, "");
    }

    #[test]
    fn lexeme_when_empty() {
        assert_eq!(Scanner::from("abc").lexeme(), "");
    }
}