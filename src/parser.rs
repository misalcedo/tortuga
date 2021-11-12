//! Parser from a stream of tokens into a syntax tree for the Tortuga language.

use crate::errors::{LexicalError, SyntaxError};
use crate::grammar::Expression;
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
    pub fn parse(self) -> Result<Expression, SyntaxError> {
        for result in self.tokens {
            match result {
                Ok(token) => println!("{:?}", token),
                Err(error) => eprintln!("{}", error),
            }
        }

        Err(SyntaxError::Unknown)
    }
}
