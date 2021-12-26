//! Performs lexical analysis on Tortuga input and produces a sequence of `Token`s.

use crate::compiler::errors::lexical::ErrorKind;
use crate::compiler::{Input, Kind, LexicalError, Token};
use std::str::Chars;

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
    type Item = Result<Token, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.input.peek()? {
            '+' => Token::new(self.input.consume(), Kind::Plus),
            '-' => Token::new(self.input.consume(), Kind::Minus),
            '*' => Token::new(self.input.consume(), Kind::Star),
            '/' => Token::new(self.input.consume(), Kind::Slash),
            '^' => Token::new(self.input.consume(), Kind::Caret),
            '=' => Token::new(self.input.consume(), Kind::Equal),
            '~' => Token::new(self.input.consume(), Kind::Tilde),
            '%' => Token::new(self.input.consume(), Kind::Percent),
            '_' => Token::new(self.input.consume(), Kind::Underscore),
            '(' => Token::new(self.input.consume(), Kind::LeftParenthesis),
            ')' => Token::new(self.input.consume(), Kind::RightParenthesis),
            '[' => Token::new(self.input.consume(), Kind::LeftBracket),
            ']' => Token::new(self.input.consume(), Kind::RightBracket),
            '{' => Token::new(self.input.consume(), Kind::LeftBrace),
            '}' => Token::new(self.input.consume(), Kind::RightBrace),
            ',' => Token::new(self.input.consume(), Kind::Comma),
            '<' => self.scan_less_than(),
            '>' => self.scan_greater_than(),
            _ => {
                return Some(Err(LexicalError::new(
                    self.input.consume(),
                    ErrorKind::Invalid,
                )))
            }
        };

        Some(Ok(token))
    }
}

impl<'a> Scanner<'a> {
    fn scan_less_than(&mut self) -> Token {
        self.input.next();

        let kind = if self.input.next_if_eq('=').is_some() {
            Kind::LessThanOrEqualTo
        } else if self.input.next_if_eq('>').is_some() {
            Kind::NotEqual
        } else {
            Kind::LessThan
        };

        Token::new(self.input.advance(), kind)
    }

    fn scan_greater_than(&mut self) -> Token {
        self.input.next();

        let kind = if self.input.next_if_eq('=').is_some() {
            Kind::GreaterThanOrEqualTo
        } else {
            Kind::GreaterThan
        };

        Token::new(self.input.advance(), kind)
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
}
