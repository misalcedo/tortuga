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
