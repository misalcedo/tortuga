//! Parse a sequence of tokens into a syntax tree.

use crate::compiler::errors::{LexicalError, Reporter};
use crate::compiler::Token;
use crate::grammar::syntax::Program;
use crate::SyntacticalError;
use std::iter::Peekable;

/// A recursive descent LL(1) parser for the syntax grammar.
/// Parses a sequence of `Token`s into syntax tree.
pub struct Parser<'a, I: Iterator<Item = Result<Token, LexicalError>>> {
    tokens: Peekable<I>,
    reporter: Reporter<'a>,
}

impl<'a, I: Iterator<Item = Result<Token, LexicalError>>> Parser<'a, I> {
    /// Creates a new `Parser`.
    pub fn new(tokens: Peekable<I>, reporter: Reporter<'a>) -> Self {
        Parser { tokens, reporter }
    }

    /// Generate a syntax tree rooted at a `Program` for this `Parser`'s sequence of tokens.
    pub fn parse(&mut self) -> Result<Program, SyntacticalError> {
        Err(SyntacticalError {})
    }
}
