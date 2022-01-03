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
    #[error("Expected value {0} to be of type {1}.")]
    UnexpectedType(Value, String),
    #[error("No definition found for function {0} with the given arguments: {}.", stringify_arguments(.1.as_slice()))]
    NoMatchingDefinition(String, Vec<Value>),
}

fn stringify_arguments(arguments: &[Value]) -> String {
    arguments
        .iter()
        .map(Value::to_string)
        .collect::<Vec<String>>()
        .join(", ")
}
