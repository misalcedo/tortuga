//! A Constraint Satisfaction Problem solver.
//! Used to determine the runtime value of a variable.
//!
//! See <https://en.wikipedia.org/wiki/Constraint_programming>

use crate::runtime::Function;
use crate::runtime::Value;
use crate::RuntimeError;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fmt;

/// The variable context for a single lexical scope.
/// Environments are a tree, the root of the tree has no parent.
/// Since all variables are immutable and variables are not allowed to shadow each other,
/// environments start as a clone of their parent.
#[derive(Clone, Debug, Default)]
pub struct Environment {
    variables: HashMap<String, Value>,
    functions: Vec<Function>,
}

/// A reference to a function.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FunctionReference(usize);

impl From<usize> for FunctionReference {
    fn from(index: usize) -> Self {
        FunctionReference(index)
    }
}

impl fmt::Display for FunctionReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{}", self.0)
    }
}

impl Environment {
    /// Creates a child [`Environment`].
    pub fn new_child(&self) -> Environment {
        self.clone()
    }

    /// Get the [`Value`] of the variable with the given [`Identifier`].
    pub fn value(&self, name: &str) -> Result<Value, RuntimeError> {
        self.variables
            .get(name)
            .copied()
            .ok_or_else(|| RuntimeError::VariableNotDefined(name.to_string()))
    }

    /// Get the [`Assignment`] of the variable with the given index.
    pub fn function(&self, reference: &FunctionReference) -> Result<Function, RuntimeError> {
        self.functions
            .get(reference.0)
            .cloned()
            .ok_or_else(|| RuntimeError::FunctionNotDefined(*reference))
    }

    /// Defines a variable as having a given [`Value`].
    /// Returns the previously defined value, if any.
    pub fn define_value(
        &mut self,
        name: Option<&str>,
        value: Value,
    ) -> Result<Value, RuntimeError> {
        match name {
            Some(name) => match self.variables.entry(name.to_string()) {
                Vacant(entry) => {
                    entry.insert(value);
                    Ok(value)
                }
                Occupied(entry) => Err(RuntimeError::VariableAlreadyDefined(
                    name.to_string(),
                    *entry.get(),
                )),
            },
            None => Ok(value),
        }
    }

    /// Defines a variable as having a given function.
    /// Returns the previously defined value as an [`Err`], if any.
    pub fn define_function(&mut self, function: Function) -> Result<Value, RuntimeError> {
        let index = self.functions.len();
        let value = FunctionReference(index).into();

        self.define_value(function.name(), value)?;
        self.functions.push(function);

        Ok(value)
    }
}
