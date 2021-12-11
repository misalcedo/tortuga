//! Interpreter errors.

use crate::grammar::ComparisonOperator;
use std::fmt;
use thiserror::Error;

/// An error that occurred while interpreting a Tortuga expression.
#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("A block must have at least one expression.")]
    EmptyBlock,
    #[error("Expected value of type {expected}, but found {actual}.")]
    InvalidType { expected: String, actual: String },
    #[error("Unable to determine whether {left} {comparison} {right}.")]
    NotComparable {
        left: String,
        comparison: ComparisonOperator,
        right: String,
    },
    #[error("Attempted to use variable '{0}' that is not yet defined.")]
    UndefinedVariableUsed(String),
    #[error("Attempted to refine variable '{0}' with the operator {1} and value {2}, but the refinement is not valid.")]
    InvalidRefinement(String, ComparisonOperator, f64),
    #[error("Attempted to define variable '{0}' as equal to {2}, but the variable is already equal to {1}.")]
    AlreadyDefined(String, f64, f64),
}

impl RuntimeError {
    pub fn invalid_type(expected: impl fmt::Display, actual: impl fmt::Display) -> Self {
        Self::InvalidType {
            expected: format!("{}", expected),
            actual: format!("{}", actual),
        }
    }

    pub fn not_comparable(
        left: impl fmt::Display,
        comparison: ComparisonOperator,
        right: impl fmt::Display,
    ) -> Self {
        Self::NotComparable {
            left: format!("{}", left),
            comparison,
            right: format!("{}", right),
        }
    }
}
