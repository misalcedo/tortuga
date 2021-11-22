//! Interpret a syntax tree to a value that can be printed to the user.

use crate::errors::RuntimeError;
use crate::grammar::*;
use crate::number::Number;
use std::convert::TryFrom;
use std::fmt;

/// Interprets a Tortuga syntax tree to a value that Rust can evaluate.
pub struct Interpreter {}

impl Interpreter {
    /// Creates a new instance of an interpreter.
    pub fn new() -> Self {
        Interpreter {}
    }

    /// Interprets a tortuga expression to a rust value.
    pub fn interpret(&self, expression: &Expression) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Grouping(grouping) => self.interpret_grouping(grouping),
            Expression::Number(number) => self.interpret_number(number),
            Expression::TextReference(text) => self.interpret_text_reference(text),
            Expression::BinaryOperation(operation) => self.interpret_binary_operation(operation),
            Expression::ComparisonOperation(operation) => {
                self.interpret_comparison_operation(operation)
            }
        }
    }

    fn interpret_grouping(&self, grouping: &Grouping) -> Result<Value, RuntimeError> {
        self.interpret(grouping.inner())
    }

    fn interpret_number(&self, number: &Number) -> Result<Value, RuntimeError> {
        Ok(Value::Number((*number).into()))
    }

    fn interpret_text_reference(&self, text: &TextReference) -> Result<Value, RuntimeError> {
        Ok(Value::TextReference(format!("{}", text)))
    }

    fn interpret_binary_operation(
        &self,
        binary_operation: &BinaryOperation,
    ) -> Result<Value, RuntimeError> {
        let left = f64::try_from(self.interpret(binary_operation.left())?)?;
        let right = f64::try_from(self.interpret(binary_operation.right())?)?;

        match binary_operation.operator() {
            Operator::Add => Ok(Value::Number(left + right)),
            Operator::Subtract => Ok(Value::Number(left - right)),
            Operator::Multiply => Ok(Value::Number(left * right)),
            Operator::Divide => Ok(Value::Number(left / right)),
        }
    }

    fn interpret_comparison_operation(
        &self,
        comparison_operation: &ComparisonOperation,
    ) -> Result<Value, RuntimeError> {
        match (
            comparison_operation.comparator(),
            self.interpret(comparison_operation.left())?,
            self.interpret(comparison_operation.right())?,
        ) {
            (comparator, Value::Number(left), Value::Number(right)) => {
                self.compare_numbers(left, comparator, right)
            }
            (comparator, Value::TextReference(left), Value::TextReference(right)) => {
                self.compare_text_references(left, comparator, right)
            }
            (ComparisonOperator::EqualTo, _, _) => Ok(Value::Boolean(false)),
            (ComparisonOperator::NotEqualTo, _, _) => Ok(Value::Boolean(true)),
            (ComparisonOperator::Comparable, _, _) => Ok(Value::Boolean(false)),
            (comparator, left, right) => Err(RuntimeError::not_comparable(left, comparator, right)),
        }
    }

    fn compare_numbers(
        &self,
        left: f64,
        comparator: ComparisonOperator,
        right: f64,
    ) -> Result<Value, RuntimeError> {
        match comparator {
            ComparisonOperator::LessThan => Ok(Value::Boolean(left < right)),
            ComparisonOperator::LessThanOrEqualTo => Ok(Value::Boolean(left <= right)),
            ComparisonOperator::GreaterThan => Ok(Value::Boolean(left > right)),
            ComparisonOperator::GreaterThanOrEqualTo => Ok(Value::Boolean(left >= right)),
            ComparisonOperator::EqualTo => Ok(Value::Boolean((left - right).abs() < f64::EPSILON)),
            ComparisonOperator::NotEqualTo => {
                Ok(Value::Boolean((left - right).abs() > f64::EPSILON))
            }
            ComparisonOperator::Comparable => Ok(Value::Boolean(true)),
        }
    }

    fn compare_text_references(
        &self,
        left: String,
        comparator: ComparisonOperator,
        right: String,
    ) -> Result<Value, RuntimeError> {
        match comparator {
            ComparisonOperator::LessThan => Ok(Value::Boolean(left < right)),
            ComparisonOperator::LessThanOrEqualTo => Ok(Value::Boolean(left <= right)),
            ComparisonOperator::GreaterThan => Ok(Value::Boolean(left > right)),
            ComparisonOperator::GreaterThanOrEqualTo => Ok(Value::Boolean(left >= right)),
            ComparisonOperator::EqualTo => Ok(Value::Boolean(left == right)),
            ComparisonOperator::NotEqualTo => Ok(Value::Boolean(left != right)),
            ComparisonOperator::Comparable => Ok(Value::Boolean(true)),
        }
    }
}

const BOOLEAN_TYPE: &str = "Boolean";
const NUMBER_TYPE: &str = "Number";
const TEXT_REFERENCE_TYPE: &str = "TextReference";

/// Represents the result of a Tortuga expression as a Rust value.
pub enum Value {
    Number(f64),
    TextReference(String),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::TextReference(text) => write!(f, "{}", text),
            Self::Boolean(value) => write!(f, "{}", value),
        }
    }
}

impl<'source> TryFrom<Value> for f64 {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Err(RuntimeError::invalid_type(NUMBER_TYPE, boolean)),
            Value::Number(number) => Ok(number),
            Value::TextReference(text) => Err(RuntimeError::invalid_type(NUMBER_TYPE, text)),
        }
    }
}

impl<'source> TryFrom<Value> for String {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => {
                Err(RuntimeError::invalid_type(TEXT_REFERENCE_TYPE, boolean))
            }
            Value::Number(number) => Err(RuntimeError::invalid_type(TEXT_REFERENCE_TYPE, number)),
            Value::TextReference(text) => Ok(text),
        }
    }
}

impl<'source> TryFrom<Value> for bool {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Ok(boolean),
            Value::Number(number) => Err(RuntimeError::invalid_type(BOOLEAN_TYPE, number)),
            Value::TextReference(text) => Err(RuntimeError::invalid_type(BOOLEAN_TYPE, text)),
        }
    }
}
