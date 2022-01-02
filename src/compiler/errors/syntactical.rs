//! Errors that may occur during syntax analysis.

use crate::compiler::Token;
use crate::LexicalError;

/// An error that occurred while generating a syntax tree from a sequence of tokens.
/// After an error is encountered, the parser may continue to generate a tree in panic mode.
#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum SyntacticalError {
    #[error("Reached the end of file prematurely; unable to complete parsing a grammar rule.")]
    Incomplete,
    #[error("No grammar rule matched the {0}.")]
    NoMatch(Token),
    #[error(transparent)]
    Lexical(#[from] LexicalError),
}

impl SyntacticalError {
    /// Tests whether the parser had complete input or ran out of tokens prematurely.
    /// [`false`] if the parser ran out of tokens. Otherwise, [`true`].
    pub fn is_complete(&self) -> bool {
        !matches!(self, Self::Incomplete)
    }
}
