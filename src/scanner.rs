//! Scans a source code file for lexical tokens.

use crate::errors::TortugaError;
use crate::token::{Location, Token, TokenKind};
use std::iter::{Iterator, Peekable};
use unicode_segmentation::UnicodeSegmentation;

/// Scanner for the tortuga language.
pub struct Scanner<'source, I>
where
    I: Iterator<Item = (usize, &'source str)>,
{
    code: &'source str,
    line: usize,
    column: usize,
    remaining: Peekable<I>,
}

/// Creates a new `Scanner` for the given source code.
pub fn new_scanner<'source>(
    code: &'source str,
) -> Scanner<'source, impl Iterator<Item = (usize, &'source str)>> {
    Scanner {
        code,
        line: 1,
        column: 1,
        remaining: code
            .grapheme_indices(true)
            .filter(|(_, grapheme)| &"\r" != grapheme)
            .peekable(),
    }
}

// Implement `Iterator` of `Token`s for `Scanner`.
impl<'source, I> Iterator for Scanner<'source, I>
where
    I: Iterator<Item = (usize, &'source str)>,
{
    // We can refer to this type using Self::Item
    type Item = Result<Token<'source>, TortugaError>;

    // Consumes the next token from the `Scanner`.
    fn next(&mut self) -> Option<Self::Item> {
        // Skip new lines.
        while matches!(self.remaining.peek(), Some((_, "\n"))) {
            self.remaining.next();
            self.line += 1;
            self.column = 1;
        }

        let next_token = match self.remaining.next() {
            None => None,
            Some((_, grapheme @ "+")) => Some(Ok(Token::new(
                TokenKind::Plus,
                grapheme,
                Location::new(self.line, (self.column, grapheme)),
            ))),
            Some((_, grapheme)) => Some(Err(TortugaError::Lexical(Location::new(
                self.line,
                (self.column, grapheme),
            )))),
        };

        // Update column.
        if let Some(Ok(ref token)) = next_token {
            self.column += token.columns();
        }

        next_token
    }
}
