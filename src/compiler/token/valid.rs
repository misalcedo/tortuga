//! A valid lexical token is a token with no errors identified during lexical analysis.

use crate::compile::{Attachment, Kind, Lexeme, LexicalToken};
use std::fmt;
use std::mem::swap;

/// A token with no lexical errors.
#[derive(Debug, PartialEq)]
pub struct ValidToken<'source> {
    lexeme: Lexeme<'source>,
    attachment: Attachment,
}

impl<'source> fmt::Display for ValidToken<'source> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} token '{}' on {}",
            self.kind(),
            self.source(),
            self.start()
        )
    }
}

impl<'source> LexicalToken<'source> for ValidToken<'source> {
    fn lexeme(&self) -> &Lexeme<'source> {
        &self.lexeme
    }
}

impl<'source> ValidToken<'source> {
    /// Creates a new instance of a `ValidToken`.
    pub fn new(attachment: Attachment, lexeme: Lexeme<'source>) -> Self {
        ValidToken { attachment, lexeme }
    }

    /// The attached data extracted during lexical analysis.
    pub fn attachment(&self) -> &Attachment {
        &self.attachment
    }

    /// The attached data extracted during lexical analysis.
    pub fn take_attachment(&mut self) -> Attachment {
        let mut attachment = Attachment::Empty((&self.attachment).into());

        swap(&mut self.attachment, &mut attachment);

        attachment
    }

    /// The kind of token that was identified during lexical analysis.
    pub fn kind(&self) -> Kind {
        self.attachment().into()
    }
}
