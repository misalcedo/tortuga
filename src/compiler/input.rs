//! Scans a source file for valid characters.
//! The Input produces a finite stream of characters, ignoring comments and blank space.

use crate::compiler::{Lexeme, Location};
use std::str::Chars;

/// Tortuga `Input` is interpreted as a sequence of Unicode code points encoded in UTF-8.
#[derive(Clone, Debug)]
pub struct Input<'source> {
    source: &'source str,
    start: Location,
    end: Location,
    peeked: Option<char>,
    characters: Chars<'source>,
}

impl Default for Input<'_> {
    fn default() -> Self {
        Input::from("")
    }
}

impl<'source> From<&'source str> for Input<'source> {
    fn from(source: &'source str) -> Self {
        Input {
            source,
            start: Location::default(),
            end: Location::default(),
            peeked: None,
            characters: source.chars(),
        }
    }
}

impl<'source> Input<'source> {
    /// Set this `Input`s start `Location` equal to its end.
    /// Resets the next lexeme to start at the current end `Location`.
    pub fn step_forward(&mut self) {
        self.start = self.end;
    }

    /// Gets the next character in the source.
    /// Skips comments, blank space, and new lines.
    pub fn next(&mut self) -> Option<char> {
        let mut c = self.peeked.or_else(|| self.characters.next())?;

        self.end.increment(c);

        let c = self.characters.next()?;

        self.end.add_column(c);

        Some(c)
    }

    /// Returns the next character only if the next one equals the expected value.
    pub fn next_if_eq(&mut self, expected: char) -> Option<char> {
        let c = self.characters.next_if_eq(&expected)?;

        self.end.add_column(c);

        Some(c)
    }

    /// Returns the next character only if the next one matches the given predicate.
    pub fn next_if(&mut self, predicate: impl FnOnce(char) -> bool) -> Option<char> {
        let c = self.characters.next_if(|c| predicate(*c))?;

        self.end.add_column(c);

        Some(c)
    }

    /// Peeks at the next character in the source.
    /// Skips any unnecessary characters before peeking.
    pub fn peek(&mut self) -> Option<char> {
        self.skip();
        self.characters.peek().copied()
    }

    /// Gets the lexeme starting at this `Input`'s start `Location` (inclusive) until this `Input`'s end `Location` (exclusive).
    pub fn lexeme(&mut self) -> Lexeme<'source> {
        let start = self.start;
        let substring = self.source.lexeme(&start, &self.end);

        self.step_forward();

        Lexeme::new(substring, start)
    }

    /// Gets the lexeme starting at the given `Location` (inclusive) until this `Input`'s end `Location` (exclusive).
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
        assert_eq!(Input::from("").peek(), None);
    }

    #[test]
    fn peek_non_empty() {
        assert_eq!(Input::from("abc").peek(), Some('a'));
    }

    #[test]
    fn peek_when_skipping() {
        assert_eq!(Input::from(";hello\r\n\t abc").peek(), Some('a'));
    }

    #[test]
    fn next_if_eq_when_equal() {
        assert_eq!(Input::from("abc").next_if_eq('a'), Some('a'));
    }

    #[test]
    fn next_if_eq_when_not_equal() {
        assert_eq!(Input::from("abc").next_if_eq('b'), None);
    }

    #[test]
    fn next_if_eq_when_empty() {
        assert_eq!(Input::from("").next_if_eq('a'), None);
    }

    #[test]
    fn next_if_when_true() {
        assert_eq!(
            Input::from("abc").next_if(|c| c.is_ascii_alphabetic()),
            Some('a')
        );
    }

    #[test]
    fn next_if_when_false() {
        assert_eq!(Input::from("abc").next_if(|c| c.is_ascii_digit()), None);
    }

    #[test]
    fn next_if_when_empty() {
        assert_eq!(Input::from("").next_if(|_| true), None);
    }

    #[test]
    fn next_normal() {
        assert_eq!(Input::from("abc").next(), Some('a'));
    }

    #[test]
    fn start_when_scanned() {
        let mut Input = Input::from("abc");

        Input.next();

        assert_eq!(*Input.start(), Location::default());
    }

    #[test]
    fn start_when_default() {
        assert_eq!(Input::from("abc").start(), &Location::default());
    }

    #[test]
    fn next_when_skipping() {
        assert_eq!(Input::from("\t abc").next(), Some('a'));
    }

    #[test]
    fn next_when_skipping_until_end() {
        assert_eq!(Input::from("; abc").next(), None);
    }

    #[test]
    fn next_when_empty() {
        assert_eq!(Input::from("").next(), None);
    }

    #[test]
    fn step_forward() {
        let mut Input = Input::from("abc");

        Input.next();
        Input.step_forward();
        Input.next_if_eq('b');

        assert_eq!(Input.lexeme(), Lexeme::new("b", Location::new(1, 2, 1)));
    }

    #[test]
    fn lexeme() {
        let mut Input = Input::from("abc");

        Input.next();
        Input.next_if_eq('b');

        let start = *Input.start();
        let first = Input.lexeme();

        assert_ne!(start, *Input.start());

        Input.next_if(|c| c.is_ascii_alphabetic());

        let second = Input.lexeme();
        let third = Input.lexeme();

        assert_eq!(first, Lexeme::new("ab", Location::new(1, 1, 0)));
        assert_eq!(second, Lexeme::new("c", Location::new(1, 3, 2)));
        assert_eq!(third, Lexeme::new("", Location::new(1, 4, 3)));
    }

    #[test]
    fn lexeme_when_empty() {
        assert_eq!(
            Input::from("abc").lexeme(),
            Lexeme::new("", Location::default())
        );
    }

    #[test]
    fn default_Input() {
        assert_eq!(Input::default(), Input::from(""))
    }
}
