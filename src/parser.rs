//! Parser from a stream of tokens into a syntax tree for the Tortuga language.

use crate::errors::{LexicalError, SyntaxError};
use crate::grammar::{Expression, Number};
use crate::scanner::TokenResult;
use crate::token::{Token, TokenKind};
use std::iter::{IntoIterator, Iterator, Peekable};

/// A recursive descent parser for a stream of tokens into a syntax tree for the Tortuga language.
pub struct Parser<'source, I: Iterator<Item = TokenResult<'source>>> {
    tokens: Peekable<I>,
}

impl<'source, I> Parser<'source, I>
where
    I: Iterator<Item = TokenResult<'source>>,
{
    /// Creates a new parser.
    pub fn new<T: IntoIterator<IntoIter = I>>(tokens: T) -> Parser<'source, I> {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Parses the stream of tokens into a syntax tree.
    pub fn parse(mut self) -> Result<Expression, SyntaxError> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
        self.parse_number().map(Expression::Number)
    }

    /// Parse a number literal with an optional plus or minus sign. 
    fn parse_number(&mut self) -> Result<Number, SyntaxError> {
        let positive = match self.peek_kind() {
            Some(TokenKind::Plus) => {
                self.advance();
                Ok(true)
            },
            Some(TokenKind::Minus) => {
                self.advance();
                Ok(false)
            },
            Some(TokenKind::Number) => Ok(true),
            _ => Err(SyntaxError::Unknown),
        };
        
        let number = self.next_match(TokenKind::Number)?;
        
        Ok(Number::new(positive?, 0, 0))
    }

    /// Advances to the next token in the stream.
    fn advance(&mut self) {
        self.tokens.next();
    }

    /// Gets the next token only if it matches the expected kind.
    fn next_match(&mut self, expected: TokenKind) -> Result<Token<'source>, SyntaxError> {
        match self.tokens.next() {
            Some(Ok(token)) if token.kind() == expected => Ok(token),
            Some(Ok(token)) => Err(SyntaxError::mismatched_kind(expected, &token)),
            Some(Err(error)) => Err(SyntaxError::Lexical(expected, error)),
            None => Err(SyntaxError::EndOfFile(expected)) 
        }
    }

    /// Peeks the token kind of the next token in the stream
    fn peek_kind(&mut self) -> Option<TokenKind> {
        match self.tokens.peek()? {
            Ok(token) => Some(token.kind()),
            Err(error) => None
        }
    }
}
