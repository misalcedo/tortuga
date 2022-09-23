//! Errors that may occur during lexical analysis.

use crate::compiler::{Excerpt, Location};
use std::fmt::{self, Display, Formatter};

/// An error that occurred during lexical analysis of a specific lexeme.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LexicalError {
    kind: LexicalErrorKind,
    excerpt: Excerpt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LexicalErrorKind {
    InvalidCodePoints,
    IdentifierStartingWithNumber,
    FractionalEndsWithZero,
    IntegerWithDotSuffix,
    IntegerWithLeadingZero,
    UnterminatedString,
}

impl Display for LexicalErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LexicalErrorKind::InvalidCodePoints => write!(f, "Invalid code points."),
            LexicalErrorKind::IdentifierStartingWithNumber => {
                write!(f, "Identifiers must not start with a number.")
            }
            LexicalErrorKind::FractionalEndsWithZero => {
                write!(f, "Fractional numbers must not end with a zero.")
            }
            LexicalErrorKind::IntegerWithDotSuffix => {
                write!(f, "Integers must not end with a dot ('.').")
            }
            LexicalErrorKind::IntegerWithLeadingZero => {
                write!(f, "Integers must not have a leading zero.")
            }
            LexicalErrorKind::UnterminatedString => write!(f, "Unterminated string."),
        }
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Encountered an error during the lexical analysis on {}. Reason: {}",
            self.excerpt, self.kind
        )
    }
}

impl std::error::Error for LexicalError {}

impl LexicalError {
    /// Creates a new instance of a [`LexicalError`].
    pub fn new(kind: LexicalErrorKind, start: Location, lexeme: &str) -> Self {
        LexicalError {
            kind,
            excerpt: Excerpt::from(start..(start + lexeme)),
        }
    }

    /// This [`LexicalError`]'s error [`LexicalErrorKind`].
    pub fn kind(&self) -> &LexicalErrorKind {
        &self.kind
    }

    /// This [`LexicalError`]'s variant.
    pub fn excerpt(&self) -> &Excerpt {
        &self.excerpt
    }
}
