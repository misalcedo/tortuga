//! Errors that may occur during syntax analysis.

use crate::compiler::Token;
use crate::LexicalError;

/// An error that occurred while generating a syntax tree from a sequence of tokens.
/// After an error is encountered, the parser may continue to generate a tree in panic mode.
#[derive(Clone, Debug, PartialEq)]
pub enum SyntacticalError {
    Incomplete,
    NoMatch(Token),
    Lexical(LexicalError),
}

impl SyntacticalError {
    /// Tests whether the parser had complete input or ran out of tokens prematurely.
    /// [`false`] if the parser ran out of tokens. Otherwise, [`true`].
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Incomplete)
    }
}

impl From<LexicalError> for SyntacticalError {
    fn from(error: LexicalError) -> Self {
        Self::Lexical(error)
    }
}
