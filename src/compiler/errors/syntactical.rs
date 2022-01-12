//! Errors that may occur during syntax analysis.

use crate::compiler::parser::Rule;

/// An error that occurred while generating a syntax tree from a sequence of tokens.
/// After an error is encountered, the parser may continue to generate a tree in panic mode.
#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum SyntacticalError {
    #[error("Reached the end of file prematurely; unable to complete parsing a grammar rule.")]
    Incomplete,
    #[error(transparent)]
    PEG(#[from] pest::error::Error<Rule>),
}

impl SyntacticalError {
    /// Tests whether the parser had complete input or ran out of tokens prematurely.
    /// [`false`] if the parser ran out of tokens. Otherwise, [`true`].
    pub fn is_complete(&self) -> bool {
        !matches!(self, Self::Incomplete)
    }
}
