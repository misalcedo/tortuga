//! Tortuga `Input` is interpreted as a sequence of Unicode code points encoded in UTF-8.

use crate::compiler::{Lexeme, Location};
use std::str::Chars;

/// Iterates input with 1 Unicode code point of lookahead.
#[derive(Clone, Debug)]
pub struct Input<I: Iterator<Item = char>> {
    start: Location,
    end: Location,
    peeked: Option<char>,
    characters: I,
}

impl<'a> From<&'a str> for Input<Chars<'a>> {
    fn from(source: &'a str) -> Self {
        Input {
            start: Location::default(),
            end: Location::default(),
            peeked: None,
            characters: source.chars(),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Input<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = if self.peeked.is_none() {
            self.characters.next()
        } else {
            self.peeked.take()
        };

        self.end.advance(c?);

        c
    }
}

impl<I: Iterator<Item = char>> Input<I> {
    /// Set this `Input`s start `Location` equal to its end.
    /// Resets the next lexeme to start at the current end `Location`.
    pub fn step_forward(&mut self) {
        self.start = self.end;
    }

    /// Lookahead by 1 Unicode code point without advancing the `Location` of the current `Lexeme`.
    pub fn peek(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.characters.next();
        }

        self.peeked
    }

    /// If the next character is equal to the `expected` value, advance the `Location` of the current `Lexeme`.
    /// Otherwise, the current `Location` is unchanged.
    pub fn next_if_eq(&mut self, expected: char) -> Option<char> {
        let c = self.peek()?;

        if c == expected {
            self.end.advance(c);
            self.peeked.take()
        } else {
            None
        }
    }

    /// Unless the next character is equal to the `avoid` value, advance the `Location` of the current `Lexeme`.
    /// Otherwise, the current `Location` is unchanged.
    pub fn next_unless_eq(&mut self, expected: char) -> Option<char> {
        let c = self.peek()?;

        if c == expected {
            None
        } else {
            self.end.advance(c);
            self.peeked.take()
        }
    }

    /// Returns the next character only if the next one matches the given predicate.
    pub fn next_if(&mut self, predicate: impl FnOnce(char) -> bool) -> Option<char> {
        let c = self.peek()?;

        if predicate(c) {
            self.end.advance(c);
            self.peeked.take()
        } else {
            None
        }
    }

    /// Gets the lexeme starting at this `Input`'s start `Location` (inclusive) until this `Input`'s end `Location` (exclusive).
    pub fn lexeme(&mut self) -> Lexeme {
        let start = self.start;

        self.start = self.end;

        Lexeme::new(start, self.end)
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
