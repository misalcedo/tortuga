//! Performs lexical analysis on Tortuga input and produces a sequence of `Token`s.

use crate::compiler::errors::lexical::ErrorKind;
use crate::compiler::number::{DECIMAL, MAX_RADIX, NUMBER_REGEX};
use crate::compiler::unicode::UnicodeProperties;
use crate::compiler::{Input, Kind, LexicalError, Token};
use std::str::Chars;

type LexicalResult = Result<Token, LexicalError>;

/// A lexical analyzer with 1 character of lookahead.
#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    input: Input<Chars<'a>>,
}

impl<'a> From<&'a str> for Scanner<'a> {
    fn from(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            input: source.into(),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = LexicalResult;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.input.skip_blank_space();

            let result = match self.input.next()? {
                '+' => self.new_token(Kind::Plus),
                '-' => self.new_token(Kind::Minus),
                '*' => self.new_token(Kind::Star),
                '/' => self.new_token(Kind::Slash),
                '^' => self.new_token(Kind::Caret),
                '=' => self.new_token(Kind::Equal),
                '~' => self.new_token(Kind::Tilde),
                '%' => self.new_token(Kind::Percent),
                '_' => self.new_token(Kind::Underscore),
                '(' => self.new_token(Kind::LeftParenthesis),
                ')' => self.new_token(Kind::RightParenthesis),
                '[' => self.new_token(Kind::LeftBracket),
                ']' => self.new_token(Kind::RightBracket),
                '{' => self.new_token(Kind::LeftBrace),
                '}' => self.new_token(Kind::RightBrace),
                ',' => self.new_token(Kind::Comma),
                ';' => {
                    self.skip_comment();
                    continue;
                }
                '<' => self.scan_less_than(),
                '>' => self.scan_greater_than(),
                '.' => self.scan_fractional_number(),
                d if d.is_ascii_digit() => self.scan_number(),
                s if s.is_xid_start() => self.scan_identifier(),
                _ => self.scan_invalid(),
            };

            return Some(result);
        }
    }
}

impl<'a> Scanner<'a> {
    fn new_token(&mut self, kind: Kind) -> Result<Token, LexicalError> {
        Ok(Token::new(self.input.advance(), kind))
    }

    fn new_error(&mut self, kind: ErrorKind) -> Result<Token, LexicalError> {
        Err(LexicalError::new(self.input.advance(), kind))
    }

    fn skip_comment(&mut self) {
        while self.input.next_unless_eq('\n').is_some() {}
    }

    fn scan_less_than(&mut self) -> LexicalResult {
        let kind = if self.input.next_if_eq('=').is_some() {
            Kind::LessThanOrEqualTo
        } else if self.input.next_if_eq('>').is_some() {
            Kind::NotEqual
        } else {
            Kind::LessThan
        };

        self.new_token(kind)
    }

    fn scan_greater_than(&mut self) -> LexicalResult {
        let kind = if self.input.next_if_eq('=').is_some() {
            Kind::GreaterThanOrEqualTo
        } else {
            Kind::GreaterThan
        };

        self.new_token(kind)
    }

    fn scan_fractional_number(&mut self) -> LexicalResult {
        let digits = self.scan_digits(DECIMAL);

        if digits == 0 {
            self.new_error(ErrorKind::Number)
        } else {
            self.new_token(Kind::Number)
        }
    }

    fn scan_number(&mut self) -> LexicalResult {
        let mut base = DECIMAL;

        let mut integer_digits = self.scan_digits(base) + 1;
        let mut fraction_digits = 0;

        if self.input.next_if_eq('#').is_some() {
            base = MAX_RADIX;
            integer_digits = self.scan_digits(base);
        }

        if self.input.next_if_eq('.').is_some() {
            fraction_digits = self.scan_digits(base);
        }

        if integer_digits == 0 && fraction_digits == 0 {
            return self.new_error(ErrorKind::Number);
        }

        let number = self.input.peek_lexeme().extract_from(self.source);

        if NUMBER_REGEX.is_match(number) {
            self.new_token(Kind::Number)
        } else {
            self.new_error(ErrorKind::Number)
        }
    }

    fn scan_digits(&mut self, radix: u32) -> usize {
        let mut digits = 0;

        while self.input.next_digit(radix).is_some() {
            digits += 1;
        }

        digits
    }

    fn scan_identifier(&mut self) -> LexicalResult {
        while self.input.next_if(|c| c.is_xid_continue()).is_some() {}
        self.new_token(Kind::Identifier)
    }

    fn scan_invalid(&mut self) -> LexicalResult {
        while self
            .input
            .next_if(|c| {
                !c.is_ascii_punctuation()
                    && !c.is_ascii_digit()
                    && !c.is_xid_start()
                    && !c.is_pattern_white_space()
            })
            .is_some()
        {}

        Err(LexicalError::new(self.input.advance(), ErrorKind::Invalid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{Lexeme, Location};

    fn validate(kind: Kind) {
        let input = kind.to_string();
        let mut scanner: Scanner<'_> = input.as_str().into();

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(
                Lexeme::new(Location::default(), input.as_str()),
                kind
            )))
        );
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scan_simple() {
        validate(Kind::Plus);
        validate(Kind::Minus);
        validate(Kind::Star);
        validate(Kind::Slash);
        validate(Kind::Percent);
        validate(Kind::Caret);
        validate(Kind::Tilde);
        validate(Kind::Equal);
        validate(Kind::NotEqual);
        validate(Kind::LessThan);
        validate(Kind::LessThanOrEqualTo);
        validate(Kind::GreaterThan);
        validate(Kind::GreaterThanOrEqualTo);
        validate(Kind::Comma);
        validate(Kind::Underscore);
        validate(Kind::LeftParenthesis);
        validate(Kind::RightParenthesis);
        validate(Kind::LeftBrace);
        validate(Kind::RightBrace);
        validate(Kind::LeftBracket);
        validate(Kind::RightBracket);
    }

    #[test]
    fn skips_invalid_characters() {
        let input = "\u{0E01EF}\u{0E01EF}\u{0E01EF}\u{0E01EF} +";
        let mut scanner: Scanner<'_> = input.into();

        let bad = Location::from(&input[..input.len() - 2]);

        assert_eq!(
            scanner.next(),
            Some(Err(LexicalError::new(
                Lexeme::new(Location::default(), bad),
                ErrorKind::Invalid
            )))
        );

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(
                Lexeme::new(&input[..input.len() - 1], input),
                Kind::Plus
            )))
        );
        assert_eq!(scanner.next(), None);
    }

    fn validate_identifier(identifier: &str) {
        let mut scanner: Scanner<'_> = identifier.into();

        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(
                Lexeme::new(Location::default(), identifier),
                Kind::Identifier
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
                Lexeme::new(Location::default(), number),
                Kind::Number
            )))
        );
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scan_number() {
        validate_number("0");
        validate_number("2");
        validate_number("21");
        validate_number("100");
        validate_number(".100");
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
        validate_number("2#.100");
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
                Lexeme::new(Location::default(), number),
                ErrorKind::Number
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
    }

    #[test]
    fn number_without_radix() {
        let input = "#1.0";
        let mut scanner: Scanner<'_> = input.into();

        assert_eq!(
            scanner.next(),
            Some(Err(LexicalError::new(
                Lexeme::new(Location::default(), "#"),
                ErrorKind::Invalid
            )))
        );
        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(Lexeme::new("#", input), Kind::Number)))
        );
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn empty_number_without_radix() {
        let input = "#.";
        let mut scanner: Scanner<'_> = input.into();

        assert_eq!(
            scanner.next(),
            Some(Err(LexicalError::new(
                Lexeme::new(Location::default(), "#"),
                ErrorKind::Invalid
            )))
        );
        assert_eq!(
            scanner.next(),
            Some(Err(LexicalError::new(
                Lexeme::new("#", input),
                ErrorKind::Number
            )))
        );
        assert_eq!(scanner.next(), None);
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
                Lexeme::new(&input[..input.len() - 2], input),
                Kind::Number
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
            Some(Ok(Token::new(
                Lexeme::new(Location::default(), "2"),
                Kind::Number
            )))
        );
        assert_eq!(
            scanner.next(),
            Some(Ok(Token::new(Lexeme::new("2", input), Kind::Identifier)))
        );
        assert_eq!(scanner.next(), None);
    }
}
