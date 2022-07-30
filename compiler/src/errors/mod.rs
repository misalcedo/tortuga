//! Errors that may occur in the compilation of Tortuga input.
//! Errors that may occur during syntax analysis.

// use crate::LexicalError;
//use crate::Location;
use std::fmt::Display;

/// An error that occurred while generating a syntax tree from a sequence of tokens.
/// After an error is encountered, the parser may continue to generate a tree in panic mode.
#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum SyntacticalError {
    #[error("Reached the end of file prematurely; unable to complete parsing a grammar rule.")]
    Incomplete,
    // #[error("No grammar rule matched the {0}.")]
    // NoMatch(String, Location),
    // #[error("Encountered multiple syntax errors.")]
    // Multiple,
    // #[error("Encountered one or more lexical errors: {}", display_slice(&.0[..], "\n\t"))]
    // Lexical(Vec<LexicalError>),
}

fn display_slice<D: Display>(items: &[D], separator: &str) -> String {
    let mut iterator = items.iter().peekable();
    let mut accumulator = String::from(separator);

    while let Some(item) = iterator.next() {
        accumulator.push_str(item.to_string().as_str());

        if iterator.peek().is_some() {
            accumulator.push_str(separator);
        }
    }

    accumulator
}

impl SyntacticalError {
    /// Tests whether the parser had complete input or ran out of tokens prematurely.
    /// [`false`] if the parser ran out of tokens. Otherwise, [`true`].
    pub fn is_complete(&self) -> bool {
        !matches!(self, Self::Incomplete)
    }
}
