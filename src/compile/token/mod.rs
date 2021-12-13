mod attachment;
mod invalid;
mod kind;
mod lexeme;
mod location;
mod valid;

pub use attachment::*;
pub use invalid::*;
pub use kind::*;
pub use lexeme::*;
pub use location::*;
pub use valid::*;

use crate::compile::LexicalError;
use std::fmt;

/// Trait for a lexical token, which contains a lexeme.
pub trait LexicalToken<'source> {
    /// The `Lexeme` for this `LexicalToken`.
    fn lexeme(&self) -> &Lexeme<'source>;

    /// The excerpt of the source file that represents this `LexicalToken`.
    fn source(&self) -> &'source str {
        self.lexeme().source()
    }

    /// The start location in the source file of this `LexicalToken`.
    fn start(&self) -> Location {
        *self.lexeme().start()
    }
}

/// A lexical token with a reference to the source.
/// The reference is used when displaying lexemes in errors.
#[derive(Debug, PartialEq)]
pub enum Token<'source> {
    Valid(ValidToken<'source>),
    Invalid(InvalidToken<'source>),
}

impl<'source> fmt::Display for Token<'source> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Valid(token) => token.fmt(f),
            Token::Invalid(token) => token.fmt(f),
        }
    }
}

impl<'source> Token<'source> {
    /// Creates a new `Token` with potential lexical errors.
    pub fn new(attachment: Attachment, lexeme: Lexeme<'source>, errors: Vec<LexicalError>) -> Self {
        if errors.is_empty() {
            Token::Valid(ValidToken::new(attachment, lexeme))
        } else {
            Token::Invalid(InvalidToken::new(Some(attachment.into()), lexeme, errors))
        }
    }

    /// Creates a valid `Token` with no lexical errors.
    pub fn new_valid(attachment: Attachment, lexeme: Lexeme<'source>) -> Self {
        Token::Valid(ValidToken::new(attachment, lexeme))
    }

    /// Creates an invalid `Token` with one or more lexical errors.
    pub fn new_invalid(
        kind: Option<Kind>,
        lexeme: Lexeme<'source>,
        errors: Vec<LexicalError>,
    ) -> Self {
        Token::Invalid(InvalidToken::new(kind, lexeme, errors))
    }
}
