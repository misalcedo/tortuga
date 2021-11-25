//! Interpret a syntax tree to a value that can be printed to the user.

use crate::errors::RuntimeError;
use crate::grammar::*;
use crate::number::Number;
use std::convert::TryFrom;
use std::fmt;
use tracing::{debug, error};

/// Interprets a Tortuga syntax tree to a value that Rust can evaluate.
#[derive(Debug)]
pub struct Interpreter {}

impl Interpreter {
    /// Creates a new instance of an interpreter.
    pub fn new() -> Self {
        Interpreter {}
    }

    /// Interprets a tortuga program to a rust value.
    pub fn interpret(&self, program: &Program) {
        debug!("Evaluating program: {}.", program);

        for expression in program.expressions() {
            debug!("Evaluating expression: {}.", expression);
            match self.interpret_expression(expression) {
                Ok(value) => println!("{}", value),
                Err(error) => error!("{}", error),
            }
        }
    }

    fn interpret_expression(&self, expression: &Expression) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Grouping(grouping) => self.interpret_grouping(grouping),
            Expression::Number(number) => self.interpret_number(number),
            Expression::BinaryOperation(operation) => self.interpret_binary_operation(operation),
            Expression::ComparisonOperation(operation) => {
                self.interpret_comparison_operation(operation)
            },
            Expression::ChainedComparisonOperation(operation) => {
                self.interpret_chained_comparison_operation(operation)
            }
        }
    }

    fn interpret_grouping(&self, grouping: &Grouping) -> Result<Value, RuntimeError> {
        self.interpret_expression(grouping.inner())
    }

    fn interpret_number(&self, number: &Number) -> Result<Value, RuntimeError> {
        Ok(Value::Number((*number).into()))
    }

    fn interpret_binary_operation(
        &self,
        binary_operation: &BinaryOperation,
    ) -> Result<Value, RuntimeError> {
        let left = f64::try_from(self.interpret_expression(binary_operation.left())?)?;
        let right = f64::try_from(self.interpret_expression(binary_operation.right())?)?;

        match binary_operation.operator() {
            Operator::Add => Ok(Value::Number(left + right)),
            Operator::Subtract => Ok(Value::Number(left - right)),
            Operator::Multiply => Ok(Value::Number(left * right)),
            Operator::Divide => Ok(Value::Number(left / right)),
            Operator::Exponent => Ok(Value::Number(left.powf(right))),
        }
    }

    fn interpret_chained_comparison_operation(
        &self,
        chained_comparison_operation: &ChainedComparisonOperation,
    ) -> Result<Value, RuntimeError> {
        let mut value = true;

        for operation in chained_comparison_operation.comparisons() {
            value = value && bool::try_from(self.interpret_comparison_operation(operation)?)?;
        }

        Ok(Value::Boolean(value))
    }

    fn interpret_comparison_operation(
        &self,
        comparison_operation: &ComparisonOperation,
    ) -> Result<Value, RuntimeError> {
        match (
            comparison_operation.comparator(),
            self.interpret_expression(comparison_operation.left())?,
            self.interpret_expression(comparison_operation.right())?,
        ) {
            (comparator, Value::Number(left), Value::Number(right)) => {
                self.compare_numbers(left, comparator, right)
            }
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
}

const BOOLEAN_TYPE: &str = "Boolean";
const NUMBER_TYPE: &str = "Number";
const TEXT_REFERENCE_TYPE: &str = "TextReference";

/// Represents the result of a Tortuga expression as a Rust value.
pub enum Value {
    Number(f64),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
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
        }
    }
}

impl<'source> TryFrom<Value> for bool {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Ok(boolean),
            Value::Number(number) => Err(RuntimeError::invalid_type(BOOLEAN_TYPE, number)),
        }
    }
}
