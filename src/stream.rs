//! Transforms an `Iterator<Item=Token<'_>>` into a stream of tokens.
//! The stream can be peeked and conditionally polled.
//! Also, transforms the infallible iterator into a fallible one.
//! That allows the parser to treat `InvalidToken`s as errors.

use crate::errors::SyntaxError;
use crate::token::{Kind, Token, ValidToken};

/// A stream of `Token`s to be consumed by the `Parser`.
pub struct TokenStream<'source, I: Iterator<Item = Token<'source>>> {
    tokens: I,
    peeked: Option<ValidToken<'source>>,
}

impl<'source, I: Iterator<Item = Token<'source>>> TokenStream<'source, I> {
    /// Creates a new instance of a `TokenStream`.
    pub fn new(tokens: I) -> Self {
        TokenStream {
            tokens,
            peeked: None,
        }
    }

    fn peek(&mut self) -> Result<Option<&ValidToken<'source>>, SyntaxError<'source>> {
        self.peeked = match self.peeked.take() {
            Some(token) => Some(token),
            None => self.next()?,
        };

        Ok(self.peeked.as_ref())
    }

    /// Consumes the next token in the `Iterator` and returns the `Token`.
    /// If the stream was peeked since the last call to `next`, the peeked value is returned instead.
    pub fn next(&mut self) -> Result<Option<ValidToken<'source>>, SyntaxError<'source>> {
        match self.peeked.take() {
            Some(token) => Ok(Some(token)),
            None => match self.tokens.next() {
                Some(Token::Valid(token)) => Ok(Some(token)),
                Some(Token::Invalid(token)) => Err(SyntaxError::InvalidToken(token)),
                None => Ok(None),
            },
        }
    }

    /// Gets the next `Token` if it matches the expected kind. Otherwise, returns an error.
    /// Returns an error if there are not more `Token`s in the stream.
    pub fn next_kind(
        &mut self,
        expected: &[Kind],
    ) -> Result<ValidToken<'source>, SyntaxError<'source>> {
        let token = self
            .next()?
            .ok_or_else(|| SyntaxError::IncompleteRule(expected.to_vec()))?;

        if expected.contains(&token.kind()) {
            Ok(token)
        } else {
            Err(SyntaxError::NoMatchingRule(token, expected.to_vec()))
        }
    }

    /// Gets the next `Token` only if it has the given kind. Otherwise, returns an empty `Option`.
    /// Returns an error if there are not more `Token`s in the stream.
    pub fn next_if_kind(
        &mut self,
        expected: &[Kind],
    ) -> Result<Option<ValidToken<'source>>, SyntaxError<'source>> {
        if expected.contains(&self.peek_kind()?) {
            self.next()
        } else {
            Ok(None)
        }
    }

    /// Peeks the next `Token`'s `Kind`.
    /// Returns an error if there are not more `Token`s in the stream.
    pub fn peek_kind(&mut self) -> Result<Kind, SyntaxError<'source>> {
        match self.peek()? {
            Some(token) => Ok(token.kind()),
            None => Err(SyntaxError::IncompleteRule(Vec::new())),
        }
    }

    /// Tests if the next token's kind matches one of the expected ones.
    pub fn next_matches_kind(&mut self, expected: &[Kind]) -> bool {
        false
    }

    /// Tests whether the `Token` stream has any more tokens without consuming any.
    pub fn is_empty(&mut self) -> bool {
        match self.peek() {
            Ok(None) => true,
            _ => false,
        }
    }
}
