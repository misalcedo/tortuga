//! Runtime representation of a function.

use crate::grammar::{self, Assignment, Parameters};
use crate::runtime::interpret::Interpret;
use crate::runtime::Environment;
use crate::{RuntimeError, Value};
use std::fmt::{self, Display, Formatter};

/// A runtime function.
#[derive(Clone, Debug)]
pub struct Function {
    code: Assignment,
    binding: Environment,
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
    pub fn new(code: &Assignment, binding: &Environment) -> Self {
        Function {
            code: code.clone(),
            binding: binding.new_child(),
        }
    }

    /// The [`Name`] patterns for this [`Function`].
    pub fn name(&self) -> Option<&str> {
        self.code.function().name().as_str()
    }

    /// The [`Parameter`] patterns for this [`Function`].
    pub fn parameters(&self) -> Option<&Parameters> {
        self.code.function().parameters()
    }

    /// Calls this [`Function`] with the given arguments.
    pub fn call(&self, arguments: &[Value]) -> Result<CallResult, RuntimeError> {
        let parameters = self.parameters();
        let mut environment = self.binding.new_child();

        match_patterns(arguments, parameters, &mut environment)?;

        let value = self.code.block().execute(&mut environment)?;

        Ok(CallResult(value, environment))
    }
}

fn match_patterns(
    arguments: &[Value],
    parameters: Option<&Parameters>,
    environment: &mut Environment,
) -> Result<(), RuntimeError> {
    match parameters {
        None if arguments.is_empty() => Ok(()),
        Some(parameters) if parameters.len() == arguments.len() => {
            for (parameter, &argument) in parameters.iter().zip(arguments.iter()) {
                let name = parameter.name().as_str();

                environment.define_value(name, argument)?;

                let pattern_matched = if name.is_none() {
                    let mut pattern_environment = environment.new_child();

                    pattern_environment.define_value(Some(String::default().as_str()), argument)?;

                    parameter.execute(&mut pattern_environment)?
                } else {
                    parameter.execute(environment)?
                };

                if let Value::Boolean(false) = pattern_matched {
                    return Err(RuntimeError::Unknown);
                }
            }

            Ok(())
        }
        _ => Err(RuntimeError::Unknown),
    }
}

impl PartialEq<grammar::Function> for Function {
    fn eq(&self, other: &grammar::Function) -> bool {
        self.code.function() == other
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}",
            self.code.function().name(),
            self.parameters().map(Parameters::len).unwrap_or_default()
        )
    }
}
