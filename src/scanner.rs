//! Scans a source code file for lexical tokens.

use crate::errors::TortugaError;
use crate::token::{Location, Token, TokenKind};
use std::iter::Iterator;
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

/// Scanner for the tortuga language.
pub struct Scanner<'source> {
    code: &'source str,
    location: Location,
    remaining: GraphemeIndices<'source>,
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` for the given source code.
    pub fn new(code: &'source str) -> Self {
        Scanner {
            code,
            location: Location::default(),
            remaining: code.grapheme_indices(true),
        }
    }
}

// Implement `Iterator` of `Token`s for `Scanner`.
impl<'source> Iterator for Scanner<'source> {
    // We can refer to this type using Self::Item
    type Item = Result<Token<'source>, TortugaError>;

    // Consumes the next token from the `Scanner`.
    fn next(&mut self) -> Option<Self::Item> {
        let next_grapheme = self.remaining.next();

        match next_grapheme {
            None => None,
            Some((index, grapheme @ "+")) => Some(Ok(Token::new(
                TokenKind::Plus,
                &self.code[index..grapheme.len()],
                self.location.bind(),
            ))),
            Some(_) => Some(Err(TortugaError::Lexical("".to_string(), self.location))),
        }
    }
}
