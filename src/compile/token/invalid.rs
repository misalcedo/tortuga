//! An invalid lexical token is a token with one or more errors identified during lexical analysis.

use crate::compile::{Kind, Lexeme, LexicalError, LexicalToken};
use std::fmt;

/// A token with one or more lexical errors.
#[derive(Debug, PartialEq)]
pub struct InvalidToken<'source> {
    kind: Option<Kind>,
    lexeme: Lexeme<'source>,
    errors: Vec<LexicalError>,
}

impl<'source> fmt::Display for InvalidToken<'source> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = self
            .kind()
            .as_ref()
            .map(Kind::to_string)
            .unwrap_or_else(|| "Indeterminate".to_string());

        write!(
            f,
            "{} token '{}' on {}, with one or more lexical errors: ",
            kind,
            self.source(),
            self.start()
        )?;

        let mut iterator = self.errors().iter().enumerate().peekable();

        while let Some((index, error)) = iterator.next() {
            write!(f, "{}) {}", index + 1, error)?;

            if iterator.peek().is_some() {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}

impl<'source> LexicalToken<'source> for InvalidToken<'source> {
    fn lexeme(&self) -> &Lexeme<'source> {
        &self.lexeme
    }
}

impl<'source> InvalidToken<'source> {
    /// Creates a new instance of a `InvalidToken`.
    pub fn new(kind: Option<Kind>, lexeme: Lexeme<'source>, errors: Vec<LexicalError>) -> Self {
        InvalidToken {
            kind,
            lexeme,
            errors,
        }
    }

    /// The list of lexical errors for this token.
    pub fn errors(&self) -> &[LexicalError] {
        self.errors.as_slice()
    }

    /// The list of lexical errors for this token.
    pub fn take_errors(&mut self) -> Vec<LexicalError> {
        self.errors.drain(..).collect()
    }

    /// The kind of token that was identified during lexical analysis.
    /// If the `Lexer` cannot determine the `Kind` of token, returns `None`.
    /// Otherwise, returns the kind that was being scanned.
    pub fn kind(&self) -> Option<Kind> {
        self.kind
    }
}
