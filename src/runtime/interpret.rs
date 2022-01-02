//! An interpreter used in the CLI prompt.

use crate::grammar::*;
use crate::runtime::{Environment, Value};
use crate::Program;
use std::ops::Deref;

/// Interprets a Tortuga [`Program`] and returns the [`Value`].
///
/// # Example
/// ## Expression
/// ```rust
/// use tortuga::{Program, Interpreter};
///
/// let program: Program = "(2 + 2#10) ^ 2".parse::<Program>().unwrap();
/// let mut interpreter = Interpreter::default();
///
/// assert_eq!(interpreter.run(program), 16.into());
/// ```
///
/// ## Comparison
/// ```rust
/// use tortuga::{Program, Interpreter};
///
/// let program: Program = "(2 + 2#10) ^ 2 = 16".parse::<Program>().unwrap();
/// let mut interpreter = Interpreter::default();
///
/// assert_eq!(interpreter.run(program), true.into());
/// ```
#[derive(Debug, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    /// Runs the given [`Program`].
    pub fn run(&mut self, program: Program) -> Value {
        program.execute(&mut self.environment).unwrap_or_default()
    }
}

/// Defines how to interpret nodes in the syntax tree.
pub trait Interpret {
    /// Interpret this node with the given [`Environment`].
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()>;
}

impl Interpret for Program {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        match self {
            Self::Expressions(expressions) => expressions.execute(environment),
            Self::Comparisons(comparisons) => comparisons.execute(environment),
        }
    }
}

impl Interpret for Expressions {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let mut value = self.head().execute(environment);

        for expression in self.tail() {
            value = expression.execute(environment);
        }

        value
    }
}

impl Interpret for Expression {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        match self {
            Self::Arithmetic(arithmetic) => arithmetic.execute(environment),
            Self::Assignment(assignment) => assignment.execute(environment),
        }
    }
}

impl Interpret for Arithmetic {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        self.epsilon().execute(environment)
    }
}

impl Interpret for Assignment {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let function = self.function();
        let name = function.name().as_str();

        let result = if function.parameters().is_none() {
            let mut local_environment = environment.new_child();
            let value = self.block().execute(&mut local_environment)?;

            environment.define_value(name, &value)
        } else {
            environment.define_function(name, self)
        };

        result.map_err(|_| ())
    }
}

impl Interpret for Epsilon {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let mut value = self.lhs().execute(environment)?;

        if let Some(rhs) = self.rhs() {
            value = crate::runtime::Epsilon::epsilon(value, rhs.execute(environment)?);
        }

        Ok(value)
    }
}

impl Interpret for Modulo {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let mut value = self.head().execute(environment)?;

        for sum in self.tail() {
            value %= sum.execute(environment)?;
        }

        Ok(value)
    }
}

impl Interpret for Sum {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let mut value = self.head().execute(environment)?;

        for add_or_subtract in self.tail() {
            match add_or_subtract {
                AddOrSubtract::Add(rhs) => value += rhs.execute(environment)?,
                AddOrSubtract::Subtract(rhs) => value -= rhs.execute(environment)?,
            }
        }

        Ok(value)
    }
}

impl Interpret for Product {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let mut value = self.head().execute(environment)?;

        for multiply_or_divide in self.tail() {
            match multiply_or_divide {
                MultiplyOrDivide::Multiply(rhs) => value *= rhs.execute(environment)?,
                MultiplyOrDivide::Divide(rhs) => value /= rhs.execute(environment)?,
            }
        }

        Ok(value)
    }
}

impl Interpret for Power {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let mut value = self.head().execute(environment)?;

        for sum in self.tail() {
            value ^= sum.execute(environment)?;
        }

        Ok(value)
    }
}

impl Interpret for Primary {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        match self {
            Primary::Number(number) => number.execute(environment),
            Primary::Call(call) => call.execute(environment),
            Primary::Grouping(grouping) => grouping.inner().execute(environment),
        }
    }
}

impl Interpret for Number {
    fn execute(&self, _: &mut Environment) -> Result<Value, ()> {
        self.number()
            .as_str()
            .parse::<crate::runtime::Number>()
            .map(Value::Number)
            .map_err(|_| ())
    }
}

impl Interpret for Call {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let name = self.identifier().as_str();
        let mut value = *environment.value(name).ok_or(())?;

        if self.arguments().is_empty() {
            return Ok(value);
        }

        for arguments in self.arguments() {
            value = call_function(&value, arguments, environment)?;
        }

        Ok(value)
    }
}

impl Interpret for Pattern {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let value = *environment
            .value(self.name().as_str().unwrap_or_default())
            .ok_or(())?;

        match self {
            Pattern::Function(function) => {
                let reference = match value {
                    _ if function.parameters().is_none() => return Ok(true.into()),
                    Value::FunctionReference(reference) => reference,
                    _ => return Err(()),
                };

                let assignment = match environment.function(&reference) {
                    None => return Err(()),
                    Some(assignment) => assignment.clone(),
                };

                Ok(Value::Boolean(assignment.function() == function.deref()))
            }
            Pattern::Refinement(refinement) => Ok(compare(
                value,
                refinement.comparator(),
                refinement.constraint().execute(environment)?,
            )),
            Pattern::Bounds(bounds) => {
                if let Value::Boolean(false) = compare_inequality(
                    bounds.left().constraint().execute(environment)?,
                    bounds.left().inequality(),
                    value,
                ) {
                    Ok(false.into())
                } else {
                    Ok(compare_inequality(
                        value,
                        bounds.right().inequality(),
                        bounds.right().constraint().execute(environment)?,
                    ))
                }
            }
        }
    }
}

impl Interpret for Comparisons {
    fn execute(&self, environment: &mut Environment) -> Result<Value, ()> {
        let mut lhs = self.lhs().execute(environment)?;
        let mut comparator = self.comparisons().head().comparator();
        let mut rhs = self.comparisons().head().rhs().execute(environment)?;

        let mut value = compare(lhs, comparator, rhs);

        for comparison in self.comparisons().tail() {
            if value == Value::Boolean(false) {
                break;
            }

            lhs = rhs;
            comparator = comparison.comparator();
            rhs = comparison.rhs().execute(environment)?;

            value &= compare(lhs, comparator, rhs);
        }

        Ok(value)
    }
}

fn compare(lhs: Value, comparator: &Comparator, rhs: Value) -> Value {
    Value::Boolean(match comparator {
        Comparator::LessThan => lhs < rhs,
        Comparator::LessThanOrEqualTo => lhs <= rhs,
        Comparator::GreaterThan => lhs > rhs,
        Comparator::GreaterThanOrEqualTo => lhs >= rhs,
        Comparator::EqualTo => lhs == rhs,
        Comparator::NotEqualTo => lhs != rhs,
    })
}

fn compare_inequality(lhs: Value, inequality: &Inequality, rhs: Value) -> Value {
    Value::Boolean(match inequality {
        Inequality::LessThan => lhs < rhs,
        Inequality::LessThanOrEqualTo => lhs <= rhs,
        Inequality::GreaterThan => lhs > rhs,
        Inequality::GreaterThanOrEqualTo => lhs >= rhs,
    })
}

fn get_assignment(value: &Value, environment: &mut Environment) -> Result<Assignment, ()> {
    let reference = match value {
        Value::FunctionReference(reference) => reference,
        _ => return Err(()),
    };

    environment.function(reference).cloned().ok_or(())
}

fn call_function(
    value: &Value,
    arguments: &Arguments,
    environment: &mut Environment,
) -> Result<Value, ()> {
    let assignment = get_assignment(value, environment)?;
    let parameters = assignment.function().parameters();

    if let Some(parameters) = parameters {
        let mut local_environment = environment.new_child();

        if parameters.len() != arguments.len() {
            return Err(());
        }

        for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
            let value = argument.execute(environment)?;
            let name = parameter.name().as_str();

            local_environment
                .define_value(name, &value)
                .map_err(|_| ())?;

            let pattern_matched = if name.is_none() {
                let mut pattern_environment = local_environment.new_child();

                pattern_environment
                    .define_value(Some(String::default().as_str()), &value)
                    .map_err(|_| ())?;

                parameter.execute(&mut pattern_environment)?
            } else {
                parameter.execute(&mut local_environment)?
            };

            if let Value::Boolean(false) = pattern_matched {
                return Err(());
            }
        }

        assignment.block().execute(&mut local_environment)
    } else {
        Err(())
    }
}
