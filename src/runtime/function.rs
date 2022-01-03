//! Runtime representation of a function.

use crate::grammar::{self, Assignment, Block, Pattern};
use crate::runtime::interpret::Interpret;
use crate::runtime::Environment;
use crate::{RuntimeError, Value};
use std::fmt::{self, Display, Formatter};

/// A declaration of a [`Function`].
#[derive(Clone, Debug)]
pub struct Declaration(Vec<Pattern>, Environment, Block);

impl Declaration {
    /// Create a new [`Declaration`].
    pub fn new(assignment: &Assignment, environment: &Environment) -> Self {
        Declaration(
            assignment.function().parameters().to_vec(),
            environment.new_child(),
            assignment.block().clone(),
        )
    }

    pub fn call(&self, arguments: &[Value]) -> Option<Result<CallResult, RuntimeError>> {
        let mut environment = self.1.new_child();

        for (parameter, &argument) in self.0.iter().zip(arguments.iter()) {
            let name = parameter.name().as_str();

            environment.define_value(name, argument).ok()?;

            let pattern_matched = if name.is_none() {
                let mut pattern_environment = environment.new_child();

                pattern_environment
                    .define_value(Some(String::default().as_str()), argument)
                    .ok()?;

                parameter.execute(&mut pattern_environment).ok()?
            } else {
                parameter.execute(&mut environment).ok()?
            };

            if let Value::Boolean(false) = pattern_matched {
                return None;
            }
        }

        Some(
            self.2
                .execute(&mut environment)
                .map(|value| CallResult(value, environment)),
        )
    }
}

/// A runtime function.
#[derive(Clone, Debug)]
pub struct Function {
    name: Option<String>,
    declarations: Vec<Declaration>,
}

/// The result of correctly invoking a [`Function`].
pub struct CallResult(Value, Environment);

impl Interpret for CallResult {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        match self.0 {
            Value::FunctionReference(reference) => {
                let function = self.1.function(&reference)?;

                environment.define_function(function)
            }
            value => Ok(value),
        }
    }
}

impl Function {
    /// Creates a new instance of a runtime [`Function`].
    pub fn new(assignment: &Assignment, environment: &Environment) -> Self {
        Function {
            name: assignment.function().name().as_str().map(String::from),
            declarations: vec![Declaration::new(assignment, environment)],
        }
    }

    /// The [`Name`] patterns for this [`Function`].
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Calls this [`Function`] with the given arguments.
    pub fn call(&self, arguments: &[Value]) -> Result<CallResult, RuntimeError> {
        for declaration in self.declarations.as_slice() {
            if let Some(result) = declaration.call(arguments) {
                return result;
            }
        }

        Err(RuntimeError::NoMatchingDefinition(
            self.to_string(),
            arguments.to_vec(),
        ))
    }
}

impl PartialEq<grammar::Function> for Function {
    fn eq(&self, other: &grammar::Function) -> bool {
        self.name() == other.name().as_str()
            && self
                .declarations
                .iter()
                .any(|declaration| declaration.0.as_slice() == other.parameters())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name().unwrap_or("_"))
    }
}
