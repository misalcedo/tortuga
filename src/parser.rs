//! Parser from a stream of tokens into a syntax tree for the Tortuga language.

use crate::errors::ParseError;
use crate::grammar::Expression;
use crate::number::{Number, Sign};
use crate::token::{Token, TokenKind};
use std::convert::TryFrom;
use std::iter::{IntoIterator, Iterator, Peekable};

/// A recursive descent parser for a stream of tokens into a syntax tree for the Tortuga language.
pub struct Parser<'source, I: Iterator<Item = Token<'source>>> {
    tokens: Peekable<I>,
}

impl<'source, I> Parser<'source, I>
where
    I: Iterator<Item = Token<'source>>,
{
    /// Creates a new parser.
    pub fn new<T: IntoIterator<IntoIter = I>>(tokens: T) -> Parser<'source, I> {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Parses the stream of tokens into a syntax tree.
    pub fn parse(mut self) -> Result<Expression, ParseError> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_number().map(Expression::Number)
    }

    /// Parse a number literal with an optional plus or minus sign.
    fn parse_number(&mut self) -> Result<Number, ParseError> {
        let sign = match self.skip_kind(&[TokenKind::Plus, TokenKind::Minus]) {
            Some(TokenKind::Minus) => Sign::Negative,
            _ => Sign::Positive,
        };

        let token = self.next_if_kind(&[TokenKind::Number])?;
        let mut number = Number::try_from(token)?;

        number.set_sign(sign);

        Ok(number)
    }

    /// Gets the next token if it matches the expected kind or returns an error.
    fn next_if_kind(&mut self, expected: &[TokenKind]) -> Result<Token<'source>, ParseError> {
        match self.tokens.next() {
            Some(token) if expected.contains(&token.kind()) => Ok(token),
            next => Err(ParseError::mismatched_kind(expected, next)),
        }
    }

    /// Skips the next token if it exists with the given kind. Does not advance the token stream.
    fn skip_kind(&mut self, expected: &[TokenKind]) -> Option<TokenKind> {
        let kind = match self.tokens.peek() {
            Some(token) if expected.contains(&token.kind()) => Some(token.kind()),
            _ => None,
        };

        if kind.is_some() {
            self.tokens.next();
        }

        kind
    }
}
