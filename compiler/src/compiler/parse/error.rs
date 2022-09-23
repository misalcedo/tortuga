//! Errors that may occur during lexical analysis.

use crate::compiler::{Excerpt, LexicalError, LexicalErrorKind};
use crate::compiler::{Location, TokenKind};
use std::fmt::{self, Display, Formatter};

/// An error that occurred during parsing of the source code's syntax tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxError {
    kind: SyntaxErrorKind,
    incomplete: bool,
    excerpt: Excerpt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyntaxErrorKind {
    CannotNegateZero,
    UnsupportedBinaryToken,
    ExpectedCurrentToken,
    NoParseRule,
    InvalidPrefixToken,
    InvalidInfixToken,
    ExpectedKind(TokenKind),
    Lexical(LexicalErrorKind),
}

impl Display for SyntaxErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxErrorKind::CannotNegateZero => write!(f, "Cannot negate zero."),
            SyntaxErrorKind::UnsupportedBinaryToken => write!(f, "Unsupported binary token."),
            SyntaxErrorKind::ExpectedCurrentToken => write!(f, "Expected current token."),
            SyntaxErrorKind::NoParseRule => write!(f, "No parse rule for the current token."),
            SyntaxErrorKind::InvalidPrefixToken => write!(f, "Unable to parse prefix token."),
            SyntaxErrorKind::InvalidInfixToken => write!(f, "Unable to parse infix token."),
            SyntaxErrorKind::ExpectedKind(kind) => write!(f, "Expected token of kind {}.", kind),
            SyntaxErrorKind::Lexical(kind) => write!(f, "{}", kind),
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error at ({}): {}", self.excerpt, self.kind)
    }
}

impl std::error::Error for SyntaxError {}

impl From<LexicalError> for SyntaxError {
    fn from(error: LexicalError) -> Self {
        SyntaxError {
            kind: SyntaxErrorKind::Lexical(*error.kind()),
            incomplete: false,
            excerpt: *error.excerpt(),
        }
    }
}

impl SyntaxError {
    pub fn is_incomplete(&self) -> bool {
        self.incomplete
    }

    pub fn new(kind: SyntaxErrorKind, excerpt: Excerpt) -> Self {
        SyntaxError {
            kind,
            incomplete: false,
            excerpt,
        }
    }

    pub fn incomplete(kind: SyntaxErrorKind, start: Location) -> Self {
        SyntaxError {
            kind,
            incomplete: true,
            excerpt: Excerpt::from(start..),
        }
    }
}
