//! An interpreter used in the CLI prompt.

use crate::grammar::*;
use crate::runtime::{Environment, EpsilonOperator, FunctionReference, Value};
use crate::{runtime, Program, RuntimeError};
use std::convert::TryFrom;
use std::ops::Deref;

/// Interprets a Tortuga [`Program`] and returns the [`Value`].
///
/// # Example
/// ## Build then run
/// ```rust
/// use tortuga::Interpreter;
///
/// let value = Interpreter::build_then_run("2*2 + (4^2 + 5^2)^.5  = 4 + 6.4 ~ 0.1");
///
/// assert_eq!(value, Ok(true.into()));
/// ```
///
/// ## Expression
/// ```rust
/// use tortuga::{Program, Interpreter};
///
/// let program: Program = "(2 + 2#10) ^ 2".parse::<Program>().unwrap();
/// let mut interpreter = Interpreter::default();
///
/// assert_eq!(interpreter.run(program), Ok(16.into()));
/// ```
///
/// ## Comparison
/// ```rust
/// use tortuga::{Program, Interpreter};
///
/// let program: Program = "(2 + 2#10) ^ 2 = 16".parse::<Program>().unwrap();
/// let mut interpreter = Interpreter::default();
///
/// assert_eq!(interpreter.run(program), Ok(true.into()));
/// ```
#[derive(Debug, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    /// Runs the given [`Program`].
    pub fn run(&mut self, program: Program) -> Result<Value, RuntimeError> {
        program.execute(&mut self.environment)
    }

    /// Build then execute the given input.
    pub fn build_then_run(source: &str) -> Result<Value, RuntimeError> {
        let program: Program = source.parse()?;

        let mut interpreter = Interpreter::default();

        interpreter.run(program)
    }
}

/// Defines how to interpret nodes in the syntax tree.
pub trait Interpret {
    /// Interpret this node with the given [`Environment`].
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError>;
}

impl Interpret for Program {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        match self {
            Self::Expressions(expressions) => expressions.execute(environment),
            Self::Comparisons(comparisons) => comparisons.execute(environment),
        }
    }
}

impl Interpret for Expressions {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let mut value = self.head().execute(environment)?;

        for expression in self.tail() {
            value = expression.execute(environment)?;
        }

        Ok(value)
    }
}

impl Interpret for Expression {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        match self {
            Self::Arithmetic(arithmetic) => arithmetic.execute(environment),
            Self::Assignment(assignment) => assignment.execute(environment),
        }
    }
}

impl Interpret for Arithmetic {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        self.epsilon().execute(environment)
    }
}

impl Interpret for Assignment {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let function = runtime::Function::new(self, environment);

        if self.function().parameters().is_empty() {
            let value = function.call(&[], environment)?.execute(environment)?;
            environment.define_value(function.name(), value)
        } else {
            environment.define_function(function)
        }
    }
}

impl Interpret for Epsilon {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let mut value = self.lhs().execute(environment)?;

        if let Some(rhs) = self.rhs() {
            value = value.epsilon(rhs.execute(environment)?);
        }

        Ok(value)
    }
}

impl Interpret for Modulo {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let mut value = self.head().execute(environment)?;

        for sum in self.tail() {
            value %= sum.execute(environment)?;
        }

        Ok(value)
    }
}

impl Interpret for Sum {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
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
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
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
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let mut value = self.head().execute(environment)?;

        for sum in self.tail() {
            value ^= sum.execute(environment)?;
        }

        Ok(value)
    }
}

impl Interpret for Primary {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        match self {
            Primary::Number(number) => number.execute(environment),
            Primary::Call(call) => call.execute(environment),
            Primary::Grouping(grouping) => grouping.inner().execute(environment),
        }
    }
}

impl Interpret for Number {
    fn execute(&self, _: &mut Environment) -> Result<Value, RuntimeError> {
        Ok(self
            .number()
            .as_str()
            .parse::<crate::runtime::Number>()
            .map(Value::Number)?)
    }
}

impl Interpret for Call {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let name = self.identifier().as_str();
        let mut value = environment.value(name)?;

        for arguments in self.arguments() {
            let reference = FunctionReference::try_from(value)?;
            let function = environment.function(&reference)?;
            let mut values = Vec::new();

            for argument in arguments.iter() {
                values.push(argument.execute(environment)?);
            }

            value = function
                .call(values.as_slice(), environment)?
                .execute(environment)?;
        }

        Ok(value)
    }
}

impl Interpret for Pattern {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let name = self.name().as_str().unwrap_or_default();
        let value = environment.value(name)?;

        match self {
            Pattern::Function(signature) => {
                if signature.parameters().is_empty() {
                    return Ok(true.into());
                }

                let reference = FunctionReference::try_from(value)?;
                let function = environment.function(&reference)?;

                Ok(Value::Boolean(&function == signature.deref()))
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
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Tolerance;

    #[test]
    fn pythagorean_function() {
        let source = r###"
            @x = 2
            @f(@a, @b) = (a^2 + b^2)^.5

            x^2 + f(4, 5) ; = 4 + 6.4 ~ 0.1
        "###;
        assert_eq!(
            Interpreter::build_then_run(source),
            Ok(Tolerance::new(10.4, 0.1).into())
        );
    }

    #[test]
    fn anonymous_parameter() {
        // Anonymous parameters cannot have conditions
        let source = r###"
            @f(_ > 3) = 42

            f(7)
        "###;
        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::NoMatchingDefinition(
                "@f".to_string(),
                vec![7.0.into()]
            ))
        );
    }

    #[test]
    fn invalid_call() {
        let source = r###"
            @x = 42
            x(7)
        "###;
        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::UnexpectedType(
                42.into(),
                "tortuga::runtime::environment::FunctionReference".to_string()
            ))
        );
    }

    #[test]
    fn no_matching_definition() {
        let source = r###"
            @f(_ > 3) = 42

            f(2)
        "###;
        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::NoMatchingDefinition(
                "@f".to_string(),
                vec![2.into()]
            ))
        );
    }

    #[test]
    fn wrong_number_of_arguments() {
        let source = r###"
            @f(_ > 3) = 42

            f(2, 4)
        "###;
        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::NoMatchingDefinition(
                "@f".to_string(),
                vec![2.into(), 4.into()]
            ))
        );
    }

    #[test]
    fn anonymous_function() {
        let source = r###"
            @f = _(@a, @b) = (a^2 + b^2)^.5
            
            f(4, 5)
        "###;
        assert_eq!(
            Interpreter::build_then_run(source),
            Ok(Tolerance::new(6.4, 0.1).into())
        );
    }

    #[test]
    fn same_function_same_patterns() {
        let source = r###"
            @f(@x) = 1
            @f(@y) = 2
            f(1) * f(2)
        "###;

        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::FunctionAlreadyDefined("@f".to_string(),))
        );
    }

    #[test]
    fn multiple_constant_functions() {
        let source = r###"
            @x = 1
            @x = 2
            x * x
        "###;

        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::FunctionAlreadyDefined("x".to_string(),))
        );
    }

    #[test]
    fn comparisons() {
        let source = "2*2 + (4^2 + 5^2)^.5  = 4 + 6.4 ~ 0.1";

        assert_eq!(Interpreter::build_then_run(source), Ok(true.into()));
    }

    #[test]
    fn recursive_factorial() {
        let source = r###"
            @factorial(@n <= 1) = 1
            @factorial(@n > 1) = n * factorial(n - 1)
            
            factorial(9)
        "###;

        assert_eq!(Interpreter::build_then_run(source), Ok(362880.into()));
    }

    #[test]
    fn variable_arity() {
        let source = r###"
            @f(@c) = c^2
            @f(@x, @y) = x * y
            
            f(2) + f(2, 2)
        "###;

        assert_eq!(Interpreter::build_then_run(source), Ok(8.into()));
    }

    #[test]
    fn constant_or_variable_function() {
        let source = r###"
            @f = 42
            @f(@c) = c^2
            @f(@x, @y) = x * y
            
            f + f(2) + f(2, 2)
        "###;

        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::UnexpectedType(
                42.into(),
                "tortuga::runtime::environment::FunctionReference".to_string()
            ))
        );
    }

    #[test]
    fn arguments_with_name_of_function() {
        let source = r###"
            @x(@x, @y) = x * y
            x(2, 2)
        "###;

        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::FunctionAlreadyDefined("x".to_string(),))
        );
    }

    #[test]
    fn arguments_with_name_of_terminal_function() {
        let source = r###"
            @x = @f(@x, @y) = x * y
            x(3, 4)
        "###;

        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::FunctionAlreadyDefined("x".to_string(),))
        );
    }

    #[test]
    fn call_internal_function() {
        let source = r###"
            @n = @f(@x, @y) = x * y
            f(3, 4)
        "###;

        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::FunctionNotDefined("f".to_string()))
        );
    }

    #[test]
    fn curry() {
        let source = r###"
            @g(@a) = @f(@x, @y) = a + x * y
            g(1)(3, 4)
        "###;

        assert_eq!(Interpreter::build_then_run(source), Ok(13.into()));
    }

    #[test]
    fn higher_order() {
        let source = r###"
            @f(@n, @callable(@n)) = callable(n^2) 
            f(2, _(@n) = n^2)
        "###;

        assert_eq!(Interpreter::build_then_run(source), Ok(8.into()));
    }

    #[test]
    fn anonymous() {
        // Groupings are not callable, so this is not parsed correctly.
        let source = r###"
            @f(@x) = (_(@n) = x + 1)(x) + (_(@n) = n + 1)(x)
            f(1)
        "###;

        assert_eq!(
            Interpreter::build_then_run(source),
            Err(RuntimeError::FunctionNotDefined("x".to_string()))
        );
    }
}
