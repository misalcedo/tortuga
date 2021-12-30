//! Report errors to users.

use crate::compiler::errors::ParseNumberError;
use crate::compiler::Lexeme;
use crate::{LexicalError, SyntacticalError};

/// Reports errors in a user-friendly format.
pub struct Reporter<'a> {
    source: &'a str,
}

impl<'a> Reporter<'a> {
    /// Resolves the given `Lexeme` to a `&str` in the source.
    fn lexeme(&self, lexeme: &Lexeme) -> &'a str {
        lexeme.extract_from(self.source)
    }

    /// Reports an error during Lexical Analysis.
    pub fn report_lexical(&self, error: LexicalError) {
        eprintln!("{:?}", error)
    }

    /// Reports an error during in generating a syntax tree.
    pub fn report_syntactical(&self, error: SyntacticalError) {
        eprintln!("{:?}", error)
    }

    /// Reports an error in parsing a numeric literal to a runtime value.
    pub fn report_parse(&self, error: ParseNumberError) {
        eprintln!("{:?}", error)
    }
}

impl<'a> From<&'a str> for Reporter<'a> {
    fn from(source: &'a str) -> Self {
        Reporter { source }
    }
}
