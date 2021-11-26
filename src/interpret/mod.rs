//! Interpret a syntax tree to a value that can be printed to the user.

mod constraints;

use crate::errors::RuntimeError;
use crate::grammar::*;
use crate::number::Number;
use constraints::ConstraintSolver;
use std::convert::TryFrom;
use std::fmt;
use tracing::{debug, error};

/// Interprets a Tortuga syntax tree to a value that Rust can evaluate.
#[derive(Debug, Default)]
pub struct Interpreter {
    solver: ConstraintSolver,
}

impl Interpreter {
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
            Expression::Variable(variable) => self.interpret_variable(variable),
            Expression::BinaryOperation(operation) => self.interpret_binary_operation(operation),
            Expression::ComparisonOperation(operation) => {
                self.interpret_comparison_operation(operation)
            }
        }
    }

    fn interpret_grouping(&self, grouping: &Grouping) -> Result<Value, RuntimeError> {
        self.interpret_expression(grouping.inner())
    }

    fn interpret_number(&self, number: &Number) -> Result<Value, RuntimeError> {
        Ok(Value::Number((*number).into()))
    }

    fn interpret_variable(&self, variable: &Variable) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.solver.value_of(variable.name())))
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
            (_,  Value::Boolean(false) | Value::Comparison(false, _, _), Value::Number(_)) => {
                Ok(Value::Boolean(false))
            },
            (_, Value::Number(_), Value::Boolean(false) | Value::Comparison(false, _, _)) => {
                Ok(Value::Boolean(false))
            }
            (comparator, Value::Comparison(true, _, left), Value::Number(right)) => {
                self.compare_numbers(left, comparator, right)
            },
            (comparator, Value::Number(left), Value::Comparison(true, right, _)) => {
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
            ComparisonOperator::LessThan => Ok(Value::Comparison(left < right, left, right)),
            ComparisonOperator::LessThanOrEqualTo => Ok(Value::Comparison(left <= right, left, right)),
            ComparisonOperator::GreaterThan => Ok(Value::Comparison(left > right, left, right)),
            ComparisonOperator::GreaterThanOrEqualTo => Ok(Value::Comparison(left >= right, left, right)),
            ComparisonOperator::EqualTo => Ok(Value::Comparison((left - right).abs() < f64::EPSILON, left, right)),
            ComparisonOperator::NotEqualTo => {
                Ok(Value::Comparison((left - right).abs() > f64::EPSILON, left, right))
            }
            ComparisonOperator::Comparable => Ok(Value::Comparison(true, left, right)),
        }
    }
}

const BOOLEAN_TYPE: &str = "Boolean";
const NUMBER_TYPE: &str = "Number";

/// Represents the result of a Tortuga expression as a Rust value.
pub enum Value {
    Number(f64),
    Boolean(bool),
    Comparison(bool, f64, f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Comparison(value, _, _) => write!(f, "{}", value),
        }
    }
}

impl<'source> TryFrom<Value> for f64 {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Err(RuntimeError::invalid_type(NUMBER_TYPE, boolean)),
            Value::Comparison(boolean, _ , _) => Err(RuntimeError::invalid_type(NUMBER_TYPE, boolean)),
            Value::Number(number) => Ok(number),
        }
    }
}

impl<'source> TryFrom<Value> for bool {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Ok(boolean),
            Value::Comparison(boolean, _ , _) => Ok(boolean),
            Value::Number(number) => Err(RuntimeError::invalid_type(BOOLEAN_TYPE, number)),
        }
    }
}
