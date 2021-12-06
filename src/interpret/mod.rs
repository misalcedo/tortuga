//! Interpret a syntax tree to a value that can be printed to the user.

mod environment;

use crate::errors::RuntimeError;
use crate::grammar::*;
use crate::number::Number;
use environment::Environment;
use std::convert::TryFrom;
use std::fmt;
use tracing::{debug, error};

/// Interprets a Tortuga syntax tree to a value that Rust can evaluate.
#[derive(Debug, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    /// Interprets a tortuga program to a rust value.
    pub fn interpret(&mut self, program: &Program) {
        debug!("Evaluating a {}.", program);

        let mut environment = self.environment.new_child();

        for expression in program.expressions() {
            match self.interpret_expression(expression, &mut environment) {
                Ok(Value::Variable(variable)) => {
                    error!("{}", RuntimeError::UndefinedVariableUsed(variable))
                }
                Ok(value) => println!("{}", value),
                Err(error) => error!("{}", error),
            }
        }
    }

    fn interpret_block(
        &mut self,
        block: &Block,
        environment: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        let mut block_environment = environment.new_child();
        let mut iterator = block.expressions().iter().peekable();
        let mut value = None;

        while let Some(expression) = iterator.next() {
            let result = self.interpret_expression(expression, &mut block_environment)?;

            if iterator.peek().is_none() {
                value.insert(result);
            }
        }

        value.ok_or(RuntimeError::EmptyBlock)
    }

    fn interpret_expression(
        &mut self,
        expression: &Expression,
        environment: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        debug!("Evaluating expression: {}.", expression);

        match expression {
            Expression::Grouping(grouping) => self.interpret_grouping(grouping, environment),
            Expression::Number(number) => self.interpret_number(number),
            Expression::Variable(variable) => self.interpret_variable(variable, environment),
            Expression::BinaryOperation(operation) => {
                self.interpret_binary_operation(operation, environment)
            }
            Expression::ComparisonOperation(operation) => {
                self.interpret_comparison_operation(operation, environment)
            }
            Expression::Block(block) => self.interpret_block(block, environment),
        }
    }

    fn interpret_grouping(
        &mut self,
        grouping: &Grouping,
        environment: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        self.interpret_expression(grouping.inner(), environment)
    }

    fn interpret_number(&self, number: &Number) -> Result<Value, RuntimeError> {
        Ok(Value::Number((*number).into()))
    }

    fn interpret_variable(
        &self,
        variable: &Variable,
        environment: &Environment,
    ) -> Result<Value, RuntimeError> {
        match environment.value_of(variable.name()) {
            Some(value) => Ok(Value::Number(value)),
            None => Ok(Value::Variable(variable.name().to_string())),
        }
    }

    fn interpret_binary_operation(
        &mut self,
        binary_operation: &BinaryOperation,
        environment: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        let left = f64::try_from(self.interpret_expression(binary_operation.left(), environment)?)?;
        let right =
            f64::try_from(self.interpret_expression(binary_operation.right(), environment)?)?;

        match binary_operation.operator() {
            Operator::Add => Ok(Value::Number(left + right)),
            Operator::Subtract => Ok(Value::Number(left - right)),
            Operator::Multiply => Ok(Value::Number(left * right)),
            Operator::Divide => Ok(Value::Number(left / right)),
            Operator::Exponent => Ok(Value::Number(left.powf(right))),
        }
    }

    fn interpret_comparison_operation(
        &mut self,
        comparison_operation: &ComparisonOperation,
        environment: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        let left = self.interpret_expression(comparison_operation.left(), environment)?;
        let right = self.interpret_expression(comparison_operation.right(), environment)?;

        self.compare_values(comparison_operation.comparator(), left, right, environment)
    }

    fn compare_values(
        &mut self,
        operator: ComparisonOperator,
        left_value: Value,
        right_value: Value,
        environment: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        match (operator, left_value, right_value) {
            (comparator, Value::Number(left), Value::Number(right)) => self.compare_numbers(
                (Value::Number(left), left),
                comparator,
                (Value::Number(right), right),
            ),
            (comparator, Value::Variable(variable), Value::Number(right)) => {
                let left = environment.refine(variable.as_str(), comparator, right)?;
                self.compare_numbers(
                    (Value::Variable(variable), left),
                    comparator,
                    (Value::Number(right), right),
                )
            }
            (comparator, Value::Number(left), Value::Variable(variable)) => {
                let right = environment.refine(variable.as_str(), comparator.flip(), left)?;
                self.compare_numbers(
                    (Value::Number(left), left),
                    comparator,
                    (Value::Variable(variable), right),
                )
            }
            (_, Value::Boolean(false) | Value::Comparison(false, _, _), Value::Number(_)) => {
                Ok(Value::Boolean(false))
            }
            (_, Value::Number(_), Value::Boolean(false) | Value::Comparison(false, _, _)) => {
                Ok(Value::Boolean(false))
            }
            (comparator, Value::Comparison(true, _, left), right @ Value::Number(_)) => {
                self.compare_values(comparator, *left, right, environment)
            }
            (comparator, left @ Value::Number(_), Value::Comparison(true, right, _)) => {
                self.compare_values(comparator, left, *right, environment)
            }
            (ComparisonOperator::Comparable, _, _) => Ok(Value::Boolean(false)),
            (comparator, left, right) => Err(RuntimeError::not_comparable(left, comparator, right)),
        }
    }

    fn compare_numbers(
        &self,
        (left_value, left): (Value, f64),
        comparator: ComparisonOperator,
        (right_value, right): (Value, f64),
    ) -> Result<Value, RuntimeError> {
        let left_value = Box::new(left_value);
        let right_value = Box::new(right_value);

        match comparator {
            ComparisonOperator::LessThan => {
                Ok(Value::Comparison(left < right, left_value, right_value))
            }
            ComparisonOperator::LessThanOrEqualTo => {
                Ok(Value::Comparison(left <= right, left_value, right_value))
            }
            ComparisonOperator::GreaterThan => {
                Ok(Value::Comparison(left > right, left_value, right_value))
            }
            ComparisonOperator::GreaterThanOrEqualTo => {
                Ok(Value::Comparison(left >= right, left_value, right_value))
            }
            ComparisonOperator::EqualTo => Ok(Value::Comparison(
                (left - right).abs() < f64::EPSILON,
                left_value,
                right_value,
            )),
            ComparisonOperator::NotEqualTo => Ok(Value::Comparison(
                (left - right).abs() > f64::EPSILON,
                left_value,
                right_value,
            )),
            ComparisonOperator::Comparable => Ok(Value::Comparison(true, left_value, right_value)),
        }
    }
}

const BOOLEAN_TYPE: &str = "Boolean";
const NUMBER_TYPE: &str = "Number";

/// Represents the result of a Tortuga expression as a Rust value.
pub enum Value {
    Number(f64),
    Boolean(bool),
    Comparison(bool, Box<Value>, Box<Value>),
    Variable(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Comparison(value, _, _) => write!(f, "{}", value),
            Self::Variable(name) => write!(f, "{}", name),
        }
    }
}

impl<'source> TryFrom<Value> for f64 {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Err(RuntimeError::invalid_type(NUMBER_TYPE, boolean)),
            Value::Comparison(boolean, _, _) => {
                Err(RuntimeError::invalid_type(NUMBER_TYPE, boolean))
            }
            Value::Number(number) => Ok(number),
            Value::Variable(name) => Err(RuntimeError::UndefinedVariableUsed(name)),
        }
    }
}

impl<'source> TryFrom<Value> for bool {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Ok(boolean),
            Value::Comparison(boolean, _, _) => Ok(boolean),
            Value::Number(number) => Err(RuntimeError::invalid_type(BOOLEAN_TYPE, number)),
            Value::Variable(name) => Err(RuntimeError::UndefinedVariableUsed(name)),
        }
    }
}
