//! Parser from a stream of tokens into a syntax tree for the Tortuga language.

use crate::token::{Token, TokenKind};
use std::iter::{IntoIterator, Iterator, Peekable};

/// A parser for a stream of tokens into a syntax tree for the Tortuga language.
pub struct Parser<'source, I: Iterator<Item=Token<'source>>> {
    tokens: Peekable<I>
}

impl<'source, I> Parser<'source, I>
where
    I: Iterator<Item=Token<'source>> 
 {
    /// Creates a new parser.
    pub fn new<T: IntoIterator<IntoIter=I>>(tokens: T) -> Parser<'source, I> {
        Parser { tokens: tokens.into_iter().peekable() }
    }
}