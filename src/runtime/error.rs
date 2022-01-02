//! Runtime errors.

use crate::runtime::FunctionReference;
use crate::Value;

/// An error that may occur while executing a [`Program`].
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("Encountered a syntax error parsing the given Tortuga input. {0}")]
    Syntax(#[from] crate::SyntacticalError),
    #[error(transparent)]
    Number(#[from] crate::ParseNumberError),
    #[error("Variable \"{0}\" is already defined as {1}.")]
    VariableAlreadyDefined(String, Value),
    #[error("Variable \"{0}\" is not defined.")]
    VariableNotDefined(String),
    #[error("Function reference \"{0}\" is not defined.")]
    FunctionNotDefined(FunctionReference),
    #[error("Encountered an unknown runtime error.")]
    Unknown,
}
