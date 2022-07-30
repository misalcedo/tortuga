//! Performs lexical analysis on Tortuga input and produces a sequence of `Token`s.

use std::borrow::Cow;
use std::str::Chars;

mod error;

use crate::{unicode::UnicodeProperties, Location, Token, TokenKind};
pub use error::LexicalError;

type LexicalResult<'a> = Result<Token<'a>, LexicalError>;

/// A lexical analyzer with 1 Unicode code point of lookahead.
#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    source: Cow<'a, str>,
    start: Location,
    end: Location,
    cursor: Chars<'a>,
}

impl<'a> From<&'a str> for Scanner<'a> {
    fn from(source: &'a str) -> Scanner<'a> {
        Scanner {
            source: source.into(),
            start: Location::default(),
            end: Location::default(),
            cursor: source.chars(),
        }
    }
}

impl From<String> for Scanner<'_> {
    fn from(source: String) -> Self {
        Scanner {
            source: source.into(),
            start: Location::default(),
            end: Location::default(),
            cursor: source.chars(),
        }
    }
}

impl<'a> TryFrom<Scanner<'a>> for Vec<Token<'a>> {
    type Error = LexicalError;

    fn try_from(scanner: Scanner<'a>) -> Result<Self, Self::Error> {
        let mut tokens = Vec::new();

        for token in scanner {
            tokens.push(token?);
        }

        Ok(tokens)
    }
}

const INVALID_CODE_POINTS: &'static str = "Invalid code points.";

impl<'a> Scanner<'a> {
    /// Returns `true` if the remaining source code starts with the given string, false otherwise.
    fn matches(&mut self, pattern: &'a str) -> bool {
        let starts_with = self.cursor.as_str().starts_with(pattern);

        if starts_with {
            if let Some(c) = self.cursor.next() {
                self.end.advance(&c);
            }
        }

        starts_with
    }

    /// Returns `true` if the remaining source code matches the given predicate, false otherwise.
    fn matches_closure<F: FnMut(char) -> bool>(&mut self, pattern: F) -> bool {
        let starts_with = self.cursor.as_str().starts_with(pattern);

        if starts_with {
            if let Some(c) = self.cursor.next() {
                self.end.advance(&c);
            }
        }

        starts_with
    }

    /// Creates a new lexical [`Token`] of the given [`TokenKind`] wrapped in a [`Result`].
    fn new_token(&mut self, kind: TokenKind) -> LexicalResult<'a> {
        let start = self.start;
        let lexeme: &'a str = &self.source[start.offset()..self.end.offset()];

        self.start = self.end;

        Ok(Token::new(start, lexeme, kind))
    }

    /// Creates a new [`LexicalError`] of the given [`ErrorKind`] wrapped in a [`Result`].
    fn new_error(&mut self, message: &str) -> LexicalResult<'a> {
        let start = self.start;
        let lexeme: &'a str = &self.source[start.offset()..self.end.offset()];

        self.start = self.end;

        Err(LexicalError::new(message, start, lexeme))
    }

    /// Skip characters until the end of the line because of a comment.
    fn skip_comment(&mut self) {
        while self.matches_closure(|c| c != '\n') {}
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

        while self.matches_closure(|c| c.is_pattern_white_space()) {}

        start != self.end
    }

    fn scan_number(&mut self, fractional: bool) -> LexicalResult<'a> {
        while self.matches_closure(|c| c.is_ascii_digit()) {}

        if !fractional && self.matches(".") {
            while self.matches_closure(|c| c.is_ascii_digit()) {}
        }

        self.new_token(TokenKind::Number)
    }

    fn scan_identifier(&mut self) -> LexicalResult<'a> {
        while self.matches_closure(|c| c.is_xid_continue()) {}
        self.new_token(TokenKind::Identifier)
    }

    fn scan_invalid(&mut self) -> LexicalResult<'a> {
        while self.matches_closure(|c| {
            !c.is_ascii_punctuation()
                && !c.is_ascii_digit()
                && !c.is_xid_start()
                && !c.is_pattern_white_space()
        }) {}

        self.new_error(INVALID_CODE_POINTS)
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = LexicalResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_blank_space();

        let result = match self.cursor.next()? {
            '0'..='9' => self.scan_number(false),
            '.' if self.matches_closure(|c| c.is_ascii_digit()) => self.scan_number(true),
            '.' => self.new_token(TokenKind::Dot),
            c if c.is_xid_start() => self.scan_identifier(),
            '(' => self.new_token(TokenKind::LeftParenthesis),
            ',' => self.new_token(TokenKind::Comma),
            ')' => self.new_token(TokenKind::RightParenthesis),
            ';' => {
                self.skip_comment();
                self.new_token(TokenKind::Semicolon)
            }
            '+' => self.new_token(TokenKind::Plus),
            '-' => self.new_token(TokenKind::Minus),
            '*' => self.new_token(TokenKind::Star),
            '/' => self.new_token(TokenKind::Slash),
            '^' => self.new_token(TokenKind::Caret),
            '=' => self.new_token(TokenKind::Equal),
            '~' => self.new_token(TokenKind::Tilde),
            '%' => self.new_token(TokenKind::Percent),
            '_' => self.new_token(TokenKind::Underscore),
            '[' => self.new_token(TokenKind::LeftBracket),
            ']' => self.new_token(TokenKind::RightBracket),
            '{' => self.new_token(TokenKind::LeftBrace),
            '}' => self.new_token(TokenKind::RightBrace),
            '@' => self.new_token(TokenKind::At),
            '!' => self.new_token(TokenKind::Exclamation),
            '|' => self.new_token(TokenKind::VerticalPipe),
            '`' => self.new_token(TokenKind::BackTick),
            '#' => self.new_token(TokenKind::Pound),
            '$' => self.new_token(TokenKind::Dollar),
            '&' => self.new_token(TokenKind::Ampersand),
            '\\' => self.new_token(TokenKind::BackSlash),
            ':' => self.new_token(TokenKind::Colon),
            '\'' => self.new_token(TokenKind::SingleQuote),
            '"' => self.new_token(TokenKind::DoubleQuote),
            '?' => self.new_token(TokenKind::Question),
            '<' if self.matches_closure(|c| c == '=') => {
                self.new_token(TokenKind::LessThanOrEqualTo)
            }
            '<' if self.matches_closure(|c| c == '>') => self.new_token(TokenKind::NotEqual),
            '<' => self.new_token(TokenKind::LessThan),
            '>' if self.matches_closure(|c| c == '=') => {
                self.new_token(TokenKind::GreaterThanOrEqualTo)
            }
            '>' => self.new_token(TokenKind::GreaterThan),
            _ => self.scan_invalid(),
        };

        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Location;

    fn validate(kind: TokenKind) {
        let input = kind.to_string();
        let mut scanner: Scanner<'_> = input.as_str().into();

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(Location::default(), input.as_str(), kind)))
        );
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scan_simple() {
        validate(TokenKind::Plus);
        validate(TokenKind::Minus);
        validate(TokenKind::Star);
        validate(TokenKind::Slash);
        validate(TokenKind::Percent);
        validate(TokenKind::Caret);
        validate(TokenKind::Tilde);
        validate(TokenKind::Equal);
        validate(TokenKind::NotEqual);
        validate(TokenKind::LessThan);
        validate(TokenKind::LessThanOrEqualTo);
        validate(TokenKind::GreaterThan);
        validate(TokenKind::GreaterThanOrEqualTo);
        validate(TokenKind::Comma);
        validate(TokenKind::Underscore);
        validate(TokenKind::At);
        validate(TokenKind::Exclamation);
        validate(TokenKind::VerticalPipe);
        validate(TokenKind::LeftParenthesis);
        validate(TokenKind::RightParenthesis);
        validate(TokenKind::LeftBrace);
        validate(TokenKind::RightBrace);
        validate(TokenKind::LeftBracket);
        validate(TokenKind::RightBracket);
    }

    #[test]
    fn skips_invalid_characters() {
        let input = "\u{0E01EF}\u{0E01EF}\u{0E01EF}\u{0E01EF} +";
        let mut scanner: Scanner<'_> = input.into();

        let bad = &input[..input.len() - 2];

        assert_eq!(
            scanner.next(),
            Some(Err(LexicalError::new(
                INVALID_CODE_POINTS,
                Location::default(),
                bad
            )))
        );

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(
                &input[..input.len() - 1],
                "+",
                TokenKind::Plus
            )))
        );
        assert_eq!(scanner.next(), None);
    }

    fn validate_identifier(identifier: &str) {
        let mut scanner: Scanner<'_> = identifier.into();

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(
                Location::default(),
                identifier,
                TokenKind::Identifier
            )))
        );
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scan_identifier() {
        validate_identifier("x");
        validate_identifier("x2");
        validate_identifier("x_2");
        validate_identifier("x__2");
        validate_identifier("xx");
        validate_identifier("x__");
        validate_identifier("x_y_z");
        validate_identifier("i");
        validate_identifier("I");
    }

    fn validate_number(number: &str) {
        let mut scanner: Scanner<'_> = number.into();

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(
                Location::default(),
                number,
                TokenKind::Number
            )))
        );
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scan_number() {
        validate_number("0");
        validate_number("2.");
        validate_number("21");
        validate_number("100");
        validate_number(".5");
        validate_number("1.0");
        validate_number("4.5");
        validate_number("0.5");
        validate_number("10000.5002");
        validate_number("7.002");

        validate_number("2#0");
        validate_number("16#F");
        validate_number("3#21");
        validate_number("2#100");
        validate_number("10#.5");
        validate_number("12#1.0");
        validate_number("20#4.5");
        validate_number("30#0.5");
        validate_number("36#10000.5002");
        validate_number("32#7.002");
        validate_number("37#1.0");
        validate_number("2#4.0");
    }

    fn invalidate_number(number: &str) {
        let mut scanner: Scanner<'_> = number.into();

        assert_eq!(
            scanner.next(),
            Some(Err(LexicalError::new(
                INVALID_CODE_POINTS,
                Location::default(),
                number,
            )))
        );
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scan_invalid_number() {
        invalidate_number(".");
        invalidate_number("20#.");
        invalidate_number("008#1.0");
        invalidate_number("0008");
        invalidate_number(".100");
        invalidate_number("2#.100");
        invalidate_number("300#1");
    }

    #[test]
    fn skip_comment() {
        let input = "; hello, world!\n \t42";

        assert_forty_two(input);
    }

    fn assert_forty_two(input: &str) {
        let mut scanner: Scanner<'_> = input.into();

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(
                &input[..input.len() - 2],
                "42",
                TokenKind::Number
            )))
        );
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn skip_multiple_comments() {
        let input = "; hello, world!\n \t; foobar\n\n42";

        assert_forty_two(input);
    }

    #[test]
    fn scan_identifier_starting_with_number() {
        let input = "2x";
        let mut scanner: Scanner<'_> = input.into();

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(Location::default(), "2", TokenKind::Number)))
        );
        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new("2", "x", TokenKind::Identifier)))
        );
        assert_eq!(scanner.next(), None);
    }
}
