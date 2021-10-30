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
        remaining: code.grapheme_indices(true).peekable(),
    }
}

/// Skips comments and new lines.
/// Returns the new location relative to the source code.
fn skip_non_tokens<'source, I>(mut start: Location, iterator: &mut Peekable<I>) -> Location
where
    I: Iterator<Item = (usize, &'source str)>,
{
    loop {
        match iterator.peek() {
            Some((_, "\r" | "\t" | " ")) => {
                iterator.next();
            }
            Some((_, "\n")) => {
                iterator.next();
                start.next_line();
            }
            Some((_, ";")) => {
                iterator.next();

                while iterator.next_if(|(_, g)| g != &"\n").is_some() {}
            }
            _ => break,
        }
    }

    start
}

/// Scans a text reference in double quotes ("...").
/// Text references can contain any grapheme except a double quote or a new line.
fn scan_text_referene<'source, I>(iterator: &mut Peekable<I>) -> Option<usize>
where
    I: Iterator<Item = (usize, &'source str)>,
{
    while iterator
        .next_if(|(_, g)| g != &"\"" && g != &"\n")
        .is_some()
    {}

    match iterator.next_if(|(_, g)| g == &"\"")? {
        (index, grapheme @ "\"") => Some(index + grapheme.len()),
        _ => None,
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
        self.location = skip_non_tokens(self.location, &mut self.remaining);

        let next_token = match self.remaining.next()? {
            (_, grapheme @ "`") => Ok((TokenKind::BackQuote, grapheme)),
            (_, grapheme @ "~") => Ok((TokenKind::Tilde, grapheme)),
            (_, grapheme @ "!") => Ok((TokenKind::Exclamation, grapheme)),
            (_, grapheme @ "@") => Ok((TokenKind::At, grapheme)),
            (_, grapheme @ "#") => Ok((TokenKind::Pound, grapheme)),
            (_, grapheme @ "$") => Ok((TokenKind::Dollar, grapheme)),
            (_, grapheme @ "%") => Ok((TokenKind::Percent, grapheme)),
            (_, grapheme @ "^") => Ok((TokenKind::Caret, grapheme)),
            (_, grapheme @ "&") => Ok((TokenKind::Ampersand, grapheme)),
            (_, grapheme @ "*") => Ok((TokenKind::Star, grapheme)),
            (_, grapheme @ "-") => Ok((TokenKind::Minus, grapheme)),
            (_, grapheme @ "_") => Ok((TokenKind::Underscore, grapheme)),
            (_, grapheme @ "=") => Ok((TokenKind::Equals, grapheme)),
            (_, grapheme @ "+") => Ok((TokenKind::Plus, grapheme)),
            (_, grapheme @ "(") => Ok((TokenKind::LeftParenthesis, grapheme)),
            (_, grapheme @ ")") => Ok((TokenKind::RightParenthesis, grapheme)),
            (_, grapheme @ "[") => Ok((TokenKind::LeftBracket, grapheme)),
            (_, grapheme @ "]") => Ok((TokenKind::RightBracket, grapheme)),
            (_, grapheme @ "{") => Ok((TokenKind::LeftBrace, grapheme)),
            (_, grapheme @ "}") => Ok((TokenKind::RightBrace, grapheme)),
            (_, grapheme @ "|") => Ok((TokenKind::Pipe, grapheme)),
            (_, grapheme @ r"\") => Ok((TokenKind::BackSlash, grapheme)),
            (_, grapheme @ ":") => Ok((TokenKind::Colon, grapheme)),
            (_, grapheme @ "?") => Ok((TokenKind::Question, grapheme)),
            (_, grapheme @ "/") => Ok((TokenKind::ForwardSlash, grapheme)),
            (_, grapheme @ "<") => Ok((TokenKind::LessThan, grapheme)),
            (_, grapheme @ ",") => Ok((TokenKind::Comma, grapheme)),
            (_, grapheme @ ">") => Ok((TokenKind::GreaterThan, grapheme)),
            (_, grapheme @ ".") => Ok((TokenKind::Period, grapheme)),
            (_, grapheme @ "'") => Ok((TokenKind::SingleQuote, grapheme)),
            (start, "\"") => match scan_text_referene(&mut self.remaining) {
                Some(end) => Ok((TokenKind::TextReference, &self.code[start..end])),
                None => Err(TortugaError::Lexical(self.location)),
            },
            _ => {
                self.location.add_columns(1);
                Err(TortugaError::Lexical(self.location))
            }
        };

        match next_token {
            Ok((kind, lexeme)) => {
                let start = self.location.clone();

                self.location.add_columns(lexeme.graphemes(true).count());

                Some(Ok(Token::new(kind, lexeme, start)))
            }
            Err(error) => Some(Err(error)),
        }
    }
}
