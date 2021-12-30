//! Errors that may occur during syntax analysis.

/// An error that occurred while generating a syntax tree from a sequence of tokens.
/// After an error is encountered, the parser may continue to generate a tree in panic mode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntacticalError {}

impl SyntacticalError {
    /// Creates a new instance of a syntax error.
    pub fn new() -> Self {
        SyntacticalError {}
    }
}
