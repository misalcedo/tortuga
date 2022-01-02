//! Runtime errors.

/// An error that may occur while executing a [`Program`].
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("Encountered a syntax error parsing the given Tortuga input. {0}")]
    Syntax(#[from] crate::SyntacticalError),
    #[error(transparent)]
    Number(#[from] crate::ParseNumberError),
    #[error("Encountered an unknown runtime error.")]
    Unknown,
}
