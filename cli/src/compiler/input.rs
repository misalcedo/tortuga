//! Tortuga `Input` is interpreted as a sequence of Unicode code points encoded in UTF-8.

use crate::compiler::unicode::UnicodeProperties;
use crate::compiler::{Lexeme, Location};
use std::str::Chars;

/// Iterates input with 1 Unicode code point of lookahead.
#[derive(Clone, Debug)]
pub struct Input<'a, I: Iterator<Item = char>> {
    source: &'a str,
    start: Location,
    end: Location,
    peeked: Option<char>,
    characters: I,
}

impl<'a> From<&'a str> for Input<'a, Chars<'a>> {
    fn from(source: &'a str) -> Self {
        Input {
            source,
            start: Location::default(),
            end: Location::default(),
            peeked: None,
            characters: source.chars(),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Input<'_, I> {
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

impl<'a, I: Iterator<Item = char>> Input<'a, I> {
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

    /// If the next character is an ASCII digit, advance the `Location` of the current `Lexeme`.
    /// Otherwise, the current `Location` is unchanged.
    pub fn next_digit(&mut self, radix: u32) -> Option<char> {
        let c = self.peek()?;

        if c.is_digit(radix) {
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

    /// Skips any blank space characters except '\n'.
    /// Returns true if any characters were skipped, false otherwise.
    ///
    /// Tortuga is a "free-form" language,
    /// meaning that all forms of whitespace serve only to separate tokens in the grammar,
    /// and have no semantic significance.
    ///
    /// A Tortuga program has identical meaning if each whitespace element is replaced with any other legal whitespace element,
    /// such as a single space character.
    ///
    /// See <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3APattern_White_Space%3A%5D&abb=on&g=&i=>
    pub fn skip_blank_space(&mut self) -> bool {
        let start = self.end;

        while self.next_if(|c| c.is_pattern_white_space()).is_some() {}

        self.start = self.end;

        start < self.end
    }

    /// Advances the `Input` to start a new `Lexeme` and returns the scanned `Lexeme`.
    pub fn advance(&mut self) -> Lexeme<'a> {
        let start = self.start;
        let lexeme = self.peek_lexeme();

        self.start = self.end;

        Lexeme::new(start, lexeme)
    }

    /// Gets the lexeme starting at this [`Input`]'s start [`Location`] (inclusive) until this [`Input`]'s end [`Location`] (exclusive).
    pub fn peek_lexeme(&self) -> &'a str {
        let start = self.start.offset();
        let end = self.end.offset();

        &self.source[start..end]
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
        let mut input = Input::from(";hello\r\n\t abc");

        while input.next_unless_eq('\n').is_some() {}
        input.skip_blank_space();

        assert_eq!(input.peek(), Some('a'));
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
    fn next_when_skipping() {
        let mut input = Input::from("\t abc");

        assert!(input.skip_blank_space());
        assert_eq!(input.next(), Some('a'));
    }

    #[test]
    fn next_when_skipping_until_end_of_line() {
        let mut input = Input::from("; abc");

        while input.next_unless_eq('\n').is_some() {}

        assert_eq!(input.next(), None);
    }

    #[test]
    fn next_when_empty() {
        assert_eq!(Input::from("").next(), None);
    }

    #[test]
    fn advance() {
        let mut input = Input::from("abc");

        input.next();
        input.advance();
        input.next_if_eq('b');

        let actual = Lexeme::new(input.start, input.peek_lexeme());
        let expected = Lexeme::new(Location::default() + "a", "b");

        assert_eq!(actual, expected);
        assert_eq!(input.advance(), expected);
    }

    #[test]
    fn lexeme() {
        let mut input = Input::from("abc");

        input.next();
        input.next_if_eq('b');

        let first = input.advance();

        input.next_if(|c| c.is_ascii_alphabetic());

        let second = input.advance();
        let third = input.advance();

        assert_eq!(first, Lexeme::new(Location::default(), "ab"));
        assert_eq!(second, Lexeme::new("ab", "c"));
        assert_eq!(third, Lexeme::new("abc", ""));
    }

    #[test]
    fn lexeme_when_empty() {
        assert_eq!(
            Input::from("abc").advance(),
            Lexeme::new(Location::default(), "")
        );
    }

    #[test]
    fn next_digit_when_true() {
        assert_eq!(Input::from("abc").next_digit(16), Some('a'));
    }

    #[test]
    fn next_digit_when_false() {
        assert_eq!(Input::from("abc").next_digit(10), None);
    }

    #[test]
    fn next_digit_when_empty() {
        assert_eq!(Input::from("").next_digit(10), None);
    }
}
