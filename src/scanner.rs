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
    location: Location,
    remaining: Peekable<I>,
}

/// Creates a new `Scanner` for the given source code.
pub fn new_scanner<'source>(
    code: &'source str,
) -> Scanner<'source, impl Iterator<Item = (usize, &'source str)>> {
    Scanner {
        code,
        location: Location::default(),
        remaining: code
            .grapheme_indices(true)
            .peekable(),
    }
}

/// Skips comments and new lines.
/// Returns the new location relative to the source code.
fn skip_non_tokens<'source, I>(mut start: Location, iterator: &mut Peekable<I>) -> Location 
    where I: Iterator<Item = (usize, &'source str)> 
{
    loop {
        match iterator.peek() {
            Some((_, "\r" | "\t" | " ")) => {
                iterator.next();
            },
            Some((_, "\n")) => {
                iterator.next();
                start.next_line();
            },
            Some((_, ";")) => {
                iterator.next();

                while iterator.next_if(|(_, g)| g != &"\n").is_some() {
                }
            },
            _ => break
        }
    }

    start
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
        self.location = skip_non_tokens(self.location, &mut self.remaining);

        let next_token = match self.remaining.next() {
            None => None,
            Some((_, grapheme @ "`")) => Some(Ok((
                TokenKind::BackQuote,
                grapheme,
            ))),
            Some((_, grapheme @ "~")) => Some(Ok((
                TokenKind::Tilde,
                grapheme,
            ))),
            Some((_, grapheme @ "!")) => Some(Ok((
                TokenKind::Exclamation,
                grapheme,
            ))),
            Some((_, grapheme @ "@")) => Some(Ok((
                TokenKind::At,
                grapheme,
            ))),
            Some((_, grapheme @ "#")) => Some(Ok((
                TokenKind::Pound,
                grapheme,
            ))),
            Some((_, grapheme @ "$")) => Some(Ok((
                TokenKind::Dollar,
                grapheme,
            ))),
            Some((_, grapheme @ "%")) => Some(Ok((
                TokenKind::Percent,
                grapheme,
            ))),
            Some((_, grapheme @ "^")) => Some(Ok((
                TokenKind::Caret,
                grapheme,
            ))),
            Some((_, grapheme @ "&")) => Some(Ok((
                TokenKind::Ampersand,
                grapheme,
            ))),
            Some((_, grapheme @ "*")) => Some(Ok((
                TokenKind::Star,
                grapheme,
            ))),
            Some((_, grapheme @ "-")) => Some(Ok((
                TokenKind::Minus,
                grapheme,
            ))),
            Some((_, grapheme @ "_")) => Some(Ok((
                TokenKind::Underscore,
                grapheme,
            ))),
            Some((_, grapheme @ "=")) => Some(Ok((
                TokenKind::Equals,
                grapheme,
            ))),
            Some((_, grapheme @ "+")) => Some(Ok((
                TokenKind::Plus,
                grapheme,
            ))),
            Some((_, grapheme @ "(")) => Some(Ok((
                TokenKind::LeftParenthesis,
                grapheme,
            ))),
            Some((_, grapheme @ ")")) => Some(Ok((
                TokenKind::RightParenthesis,
                grapheme,
            ))),
            Some((_, grapheme @ "[")) => Some(Ok((
                TokenKind::LeftBracket,
                grapheme,
            ))),
            Some((_, grapheme @ "]")) => Some(Ok((
                TokenKind::RightBracket,
                grapheme,
            ))),
            Some((_, grapheme @ "{")) => Some(Ok((
                TokenKind::LeftBrace,
                grapheme,
            ))),
            Some((_, grapheme @ "}")) => Some(Ok((
                TokenKind::RightBrace,
                grapheme,
            ))),
            Some((_, grapheme @ "|")) => Some(Ok((
                TokenKind::Pipe,
                grapheme,
            ))),
            Some((_, grapheme @ r"\")) => Some(Ok((
                TokenKind::BackSlash,
                grapheme,
            ))),
            Some((_, grapheme @ ":")) => Some(Ok((
                TokenKind::Colon,
                grapheme,
            ))),
            Some((_, grapheme @ "?")) => Some(Ok((
                TokenKind::Question,
                grapheme,
            ))),
            Some((_, grapheme @ "/")) => Some(Ok((
                TokenKind::ForwardSlash,
                grapheme,
            ))),
            Some((_, grapheme @ "<")) => Some(Ok((
                TokenKind::LessThan,
                grapheme,
            ))),
            Some((_, grapheme @ ",")) => Some(Ok((
                TokenKind::Comma,
                grapheme,
            ))),
            Some((_, grapheme @ ">")) => Some(Ok((
                TokenKind::GreaterThan,
                grapheme,
            ))),
            Some((_, grapheme @ ".")) => Some(Ok((
                TokenKind::Period,
                grapheme,
            ))),
            Some((_, grapheme @ "'")) => Some(Ok((
                TokenKind::SingleQuote,
                grapheme,
            ))),
            Some((_, grapheme @ "\"")) => Some(Ok((
                TokenKind::DoubleQuote,
                grapheme,
            ))),
            Some(_) => Some(Err(TortugaError::Lexical(self.location))),
        };

        match next_token {
            None => None,
            Some(Ok((kind, lexeme))) => {
                let start = self.location.clone();
                
                self.location.add_columns(lexeme.graphemes(true).count());
                
                Some(Ok(Token::new(kind, lexeme, start)))
            },
            Some(Err(error)) => {
                self.location.add_columns(1);
                Some(Err(error))
            }
        }
    }
}
