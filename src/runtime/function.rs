//! Runtime representation of a function.

use crate::grammar::{self, Assignment, Block, Pattern};
use crate::runtime::interpret::Interpret;
use crate::runtime::Environment;
use crate::{RuntimeError, Value};
use std::fmt::{self, Display, Formatter, Write};

/// A declaration of a [`Function`].
#[derive(Clone, Debug)]
pub struct Declaration(Vec<Pattern>, Environment, Body);

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Block(Block),
    Value(Value),
}

impl From<Block> for Body {
    fn from(block: Block) -> Self {
        Body::Block(block)
    }
}

impl From<Value> for Body {
    fn from(value: Value) -> Self {
        Body::Value(value)
    }
}

impl Interpret for Body {
    fn execute(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        match self {
            Body::Block(block) => block.execute(environment),
            Body::Value(value) => Ok(*value),
        }
    }
}

impl Declaration {
    /// Create a new [`Declaration`].
    pub fn new(assignment: &Assignment, environment: &Environment) -> Self {
        Declaration(
            assignment.function().parameters().to_vec(),
            environment.clone(),
            assignment.block().clone().into(),
        )
    }

    /// Create a new constant [`Declaration`].
    pub fn new_constant(value: Value, environment: &Environment) -> Self {
        Declaration(Vec::new(), environment.clone(), value.into())
    }

    pub fn call(
        &self,
        arguments: &[Value],
        environment: &mut Environment,
    ) -> Option<Result<CallResult, RuntimeError>> {
        if !self.0.is_empty() && self.0.len() != arguments.len() {
            return None;
        }

        let mut local_environment = self.1.clone();

        for (parameter, &argument) in self.0.iter().zip(arguments.iter()) {
            let name = parameter.name().as_str();

            local_environment
                .define_function_from(environment, name, argument)
                .ok()?;

            if let Value::Boolean(false) = parameter.execute(&mut local_environment).ok()? {
                return None;
            }
        }

        Some(
            self.2
                .execute(&mut local_environment)
                .map(|value| CallResult(value, local_environment)),
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

                environment.define_function(function).map(Value::from)
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

    /// Creates a new instance of a runtime [`Function`].
    pub fn new_constant(name: Option<&str>, value: Value, environment: &Environment) -> Self {
        Function {
            name: name.map(String::from),
            declarations: vec![Declaration::new_constant(value, environment)],
        }
    }

    /// The [`Name`] patterns for this [`Function`].
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Calls this [`Function`] with the given arguments.
    pub fn call(
        &self,
        arguments: &[Value],
        environment: &mut Environment,
    ) -> Result<CallResult, RuntimeError> {
        for declaration in self.declarations.as_slice() {
            if let Some(result) = declaration.call(arguments, environment) {
                return result;
            }
        }

        Err(RuntimeError::NoMatchingDefinition(
            self.to_string(),
            arguments.to_vec(),
        ))
    }

    /// Merges the declarations of the given function into this one.
    pub fn merge(&mut self, other: Function) -> Result<(), RuntimeError> {
        if self == &other {
            return Err(RuntimeError::FunctionAlreadyDefined(self.to_string()));
        }

        for declaration in other.declarations {
            self.declarations.push(declaration);
        }

        Ok(())
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
            && self.declarations.iter().any(|a| {
                other
                    .declarations
                    .iter()
                    .any(|b| a.0.as_slice() == b.0.as_slice())
            })
    }
}

impl PartialEq<grammar::Function> for Function {
    fn eq(&self, other: &grammar::Function) -> bool {
        self.declarations
            .iter()
            .any(|declaration| declaration.0.as_slice() == other.parameters())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => write!(f, "@{}", name),
            None => f.write_char('_'),
        }
    }
}
