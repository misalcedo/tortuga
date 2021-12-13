//! Scans a source file for valid characters.
//! The scanner produces a finite stream of characters, ignoring comments and blank space.

use crate::compile::{Lexeme, LexemeSource, Location};
use std::iter::Peekable;
use std::str::Chars;

/// Scans a source text until completion.
/// Skips comments, new lines, and blank space.
/// Assumes the source code is written left to write.
#[derive(Clone, Debug)]
pub struct Scanner<'source> {
    source: &'source str,
    start: Location,
    end: Location,
    cursor: Peekable<Chars<'source>>,
}

impl Default for Scanner<'_> {
    fn default() -> Self {
        Scanner::from("")
    }
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
            cursor: source.chars().peekable(),
        }
    }
}

impl<'source> Scanner<'source> {
    /// Skips comments until the end of the current line.
    fn skip_comment(&mut self) {
        while let Some(c) = self.cursor.next_if(|c| c != &'\n') {
            self.end.add_column(c);
        }
    }

    /// Set this `Scanner`s start `Location` equal to its end.
    /// Resets the next lexeme to start at the current end `Location`.
    pub fn step_forward(&mut self) {
        self.start = self.end;
    }

    /// Skips any tokens are not meant to be part of a lexeme.
    fn skip(&mut self) {
        loop {
            match self
                .cursor
                .next_if(|c| c == &'\r' || c == &'\n' || c == &'\t' || c == &' ' || c == &';')
            {
                Some(';') => self.skip_comment(),
                Some('\n') => self.end.next_line(),
                Some(c @ '\r') => self.end.add_offset(c),
                Some(c @ ('\t' | ' ')) => self.end.add_column(c),
                _ => break,
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
        let c = self.cursor.next_if_eq(&expected)?;

        self.end.add_column(c);

        Some(c)
    }

    /// Returns the next character only if the next one matches the given predicate.
    pub fn next_if(&mut self, predicate: impl FnOnce(char) -> bool) -> Option<char> {
        let c = self.cursor.next_if(|c| predicate(*c))?;

        self.end.add_column(c);

        Some(c)
    }

    /// Peeks at the next character in the source.
    /// Skips any unnecessary characters before peeking.
    pub fn peek(&mut self) -> Option<char> {
        self.skip();
        self.cursor.peek().copied()
    }

    /// Gets the lexeme starting at this `Scanner`'s start `Location` (inclusive) until this `Scanner`'s end `Location` (exclusive).
    pub fn lexeme(&mut self) -> Lexeme<'source> {
        let start = self.start;
        let substring = self.source.lexeme(&start, &self.end);

        self.step_forward();

        Lexeme::new(substring, start)
    }

    /// Gets the lexeme starting at the given `Location` (inclusive) until this `Scanner`'s end `Location` (exclusive).
    pub fn lexeme_from(&mut self, start: &Location) -> Lexeme<'source> {
        Lexeme::new(self.source.lexeme(start, &self.end), *start)
    }

    /// The start location of the current lexeme being scanned.
    /// Used to scan multi-part tokens (e.g., numeric literals), so the lexeme covers the entire token.
    pub fn start(&self) -> &Location {
        &self.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_empty() {
        assert_eq!(Scanner::from("").peek(), None);
    }

    #[test]
    fn peek_non_empty() {
        assert_eq!(Scanner::from("abc").peek(), Some('a'));
    }

    #[test]
    fn peek_when_skipping() {
        assert_eq!(Scanner::from(";hello\r\n\t abc").peek(), Some('a'));
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
        assert_eq!(
            Scanner::from("abc").next_if(|c| c.is_ascii_alphabetic()),
            Some('a')
        );
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
    fn start_when_scanned() {
        let mut scanner = Scanner::from("abc");

        scanner.next();

        assert_eq!(*scanner.start(), Location::default());
    }

    #[test]
    fn start_when_default() {
        assert_eq!(Scanner::from("abc").start(), &Location::default());
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
    fn step_forward() {
        let mut scanner = Scanner::from("abc");

        scanner.next();
        scanner.step_forward();
        scanner.next_if_eq('b');

        assert_eq!(scanner.lexeme(), Lexeme::new("b", Location::new(1, 2, 1)));
    }

    #[test]
    fn lexeme() {
        let mut scanner = Scanner::from("abc");

        scanner.next();
        scanner.next_if_eq('b');

        let start = *scanner.start();
        let first = scanner.lexeme();

        assert_ne!(start, *scanner.start());

        scanner.next_if(|c| c.is_ascii_alphabetic());

        let second = scanner.lexeme();
        let third = scanner.lexeme();

        assert_eq!(first, Lexeme::new("ab", Location::new(1, 1, 0)));
        assert_eq!(second, Lexeme::new("c", Location::new(1, 3, 2)));
        assert_eq!(third, Lexeme::new("", Location::new(1, 4, 3)));
    }

    #[test]
    fn lexeme_when_empty() {
        assert_eq!(
            Scanner::from("abc").lexeme(),
            Lexeme::new("", Location::default())
        );
    }

    #[test]
    fn default_scanner() {
        assert_eq!(Scanner::default(), Scanner::from(""))
    }
}
