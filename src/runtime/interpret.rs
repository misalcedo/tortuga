//! An interpreter used in the CLI prompt.

use crate::grammar::*;
use crate::runtime::{Environment, Value};
use crate::Program;

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
        program.execute(&mut self.environment)
    }
}

/// Defines how to interpret nodes in the syntax tree.
pub trait Interpret {
    /// Interpret this node with the given [`Environment`].
    fn execute(&self, environment: &mut Environment) -> Value;
}

impl Interpret for Program {
    fn execute(&self, environment: &mut Environment) -> Value {
        match self {
            Self::Expressions(expressions) => expressions.execute(environment),
            Self::Comparisons(comparisons) => comparisons.execute(environment),
        }
    }
}

impl Interpret for Expressions {
    fn execute(&self, environment: &mut Environment) -> Value {
        let mut value = self.head().execute(environment);

        for expression in self.tail() {
            value = expression.execute(environment);
        }

        value
    }
}

impl Interpret for Expression {
    fn execute(&self, environment: &mut Environment) -> Value {
        match self {
            Self::Arithmetic(arithmetic) => arithmetic.execute(environment),
            Self::Assignment(assignment) => assignment.execute(environment),
        }
    }
}

impl Interpret for Arithmetic {
    fn execute(&self, environment: &mut Environment) -> Value {
        self.epsilon().execute(environment)
    }
}

impl Interpret for Assignment {
    fn execute(&self, _: &mut Environment) -> Value {
        Value::Unit
    }
}

impl Interpret for Epsilon {
    fn execute(&self, environment: &mut Environment) -> Value {
        let mut value = self.lhs().execute(environment);

        if let Some(rhs) = self.rhs() {
            value = value.epsilon(rhs.execute(environment));
        }

        value
    }
}

impl Interpret for Modulo {
    fn execute(&self, environment: &mut Environment) -> Value {
        let mut value = self.head().execute(environment);

        for sum in self.tail() {
            value = value % sum.execute(environment);
        }

        value
    }
}

impl Interpret for Sum {
    fn execute(&self, environment: &mut Environment) -> Value {
        let mut value = self.head().execute(environment);

        for add_or_subtract in self.tail() {
            match add_or_subtract {
                AddOrSubtract::Add(rhs) => value = value + rhs.execute(environment),
                AddOrSubtract::Subtract(rhs) => value = value - rhs.execute(environment),
            }
        }

        value
    }
}

impl Interpret for Product {
    fn execute(&self, environment: &mut Environment) -> Value {
        let mut value = self.head().execute(environment);

        for multiply_or_divide in self.tail() {
            match multiply_or_divide {
                MultiplyOrDivide::Multiply(rhs) => value = value * rhs.execute(environment),
                MultiplyOrDivide::Divide(rhs) => value = value / rhs.execute(environment),
            }
        }

        value
    }
}

impl Interpret for Power {
    fn execute(&self, environment: &mut Environment) -> Value {
        let mut value = self.head().execute(environment);

        for sum in self.tail() {
            value = value ^ sum.execute(environment);
        }

        value
    }
}

impl Interpret for Primary {
    fn execute(&self, environment: &mut Environment) -> Value {
        match self {
            Primary::Number(number) => number.execute(environment),
            Primary::Call(call) => call.execute(environment),
            Primary::Grouping(grouping) => grouping.inner().execute(environment),
        }
    }
}

impl Interpret for Number {
    fn execute(&self, _: &mut Environment) -> Value {
        self.number()
            .as_str()
            .parse::<crate::runtime::Number>()
            .map(Value::Number)
            .unwrap_or(Value::Unit)
    }
}

impl Interpret for Call {
    fn execute(&self, _: &mut Environment) -> Value {
        Value::Unit
    }
}

impl Interpret for Comparisons {
    fn execute(&self, environment: &mut Environment) -> Value {
        let mut lhs = self.lhs().execute(environment);
        let mut comparator = self.comparisons().head().comparator();
        let mut rhs = self.comparisons().head().rhs().execute(environment);

        let mut value = compare(lhs, comparator, rhs);

        for comparison in self.comparisons().tail() {
            if value == Value::Boolean(false) {
                break;
            }

            lhs = rhs;
            comparator = comparison.comparator();
            rhs = comparison.rhs().execute(environment);

            value = value & compare(lhs, comparator, rhs);
        }

        value
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
