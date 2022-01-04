//! Runtime errors.

use crate::Value;

/// An error that may occur while executing a [`Program`].
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("Encountered a syntax error parsing the given Tortuga input. {0}")]
    Syntax(#[from] crate::SyntacticalError),
    #[error(transparent)]
    Number(#[from] crate::ParseNumberError),
    #[error("Function {0} is already defined.")]
    FunctionAlreadyDefined(String),
    #[error("Function @{0} is not defined.")]
    FunctionNotDefined(String),
    #[error("Expected value {0} to be of type {1}.")]
    UnexpectedType(Value, String),
    #[error("No definition found for function @{0} with the given arguments: {}.", stringify_arguments(.1.as_slice()))]
    NoMatchingDefinition(String, Vec<Value>),
}

fn stringify_arguments(arguments: &[Value]) -> String {
    arguments
        .iter()
        .map(Value::to_string)
        .collect::<Vec<String>>()
        .join(", ")
}
