//! Errors that may occur during syntax analysis.

/// An error that occurred while generating a syntax tree from a sequence of tokens.
/// After an error is encountered, the parser may continue to generate a tree in panic mode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntacticalError {
    kind: ErrorKind,
}

impl SyntacticalError {
    /// Tests whether the parser had complete input or ran out of tokens prematurely.
    /// [`false`] if the parser ran out of tokens. Otherwise, [`true`].
    pub fn is_complete(&self) -> bool {
        self.kind != ErrorKind::Incomplete
    }
}

/// The kind of syntax error that occurred.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Incomplete,
    NoMatch,
    Lexical,
}

impl From<ErrorKind> for SyntacticalError {
    fn from(kind: ErrorKind) -> Self {
        SyntacticalError { kind }
    }
}
