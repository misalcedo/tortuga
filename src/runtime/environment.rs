//! A scope used to determine the runtime value of a function.

use crate::runtime::Function;
use crate::{RuntimeError, Value};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fmt;

/// The variable context for a single lexical scope.
/// Environments are a tree, the root of the tree has no parent.
/// Since all variables are immutable and variables are not allowed to shadow each other,
/// environments start as a clone of their parent.
#[derive(Clone, Debug, Default)]
pub struct Environment {
    names: HashMap<String, Value>,
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
        write!(f, "{}", self.0)
    }
}

impl Environment {
    /// Get the [`Function`] declarations with the given name.
    pub fn function(&self, reference: &FunctionReference) -> Result<Function, RuntimeError> {
        self.functions
            .get(reference.0)
            .cloned()
            .ok_or_else(|| RuntimeError::FunctionNotDefined(reference.to_string()))
    }

    /// Get the [`FunctionReference`] with the given name.
    pub fn value(&self, name: &str) -> Result<Value, RuntimeError> {
        self.names
            .get(name)
            .copied()
            .ok_or_else(|| RuntimeError::FunctionNotDefined(name.to_string()))
    }

    pub fn override_function(&mut self, function: Function) -> Result<Value, RuntimeError> {
        if let Some(name) = function.name() {
            if let Some(Value::FunctionReference(reference)) = self.names.get(name) {
                if let Some(slot) = self.functions.get_mut(reference.0) {
                    *slot = function;
                    return Ok(Value::from(*reference));
                }
            }
        }

        self.define_function(function)
    }

    /// Defines a [`Function`] as having a given name.
    /// Returns the previously defined value as an [`Err`], if any.
    pub fn define_function(&mut self, function: Function) -> Result<Value, RuntimeError> {
        if let Some(name) = function.name() {
            if let Some(Value::FunctionReference(reference)) = self.names.get(name) {
                let existing = self
                    .functions
                    .get_mut(reference.0)
                    .ok_or_else(|| RuntimeError::FunctionNotDefined(name.to_string()))?;

                existing.merge(function)?;

                return Ok(Value::from(*reference));
            }
        }

        let index = self.functions.len();
        let value = FunctionReference(index).into();

        self.define_value(function.name(), value)?;
        self.functions.push(function);

        Ok(value)
    }

    /// Defines a variable as having a given [`Value`].
    /// Returns the previously defined value, if any.
    pub fn define_value(
        &mut self,
        name: Option<&str>,
        value: Value,
    ) -> Result<Value, RuntimeError> {
        match name {
            Some(name) => match self.names.entry(name.to_string()) {
                Vacant(entry) => {
                    entry.insert(value);
                    Ok(value)
                }
                Occupied(_) => Err(RuntimeError::FunctionAlreadyDefined(format!("@{}", name))),
            },
            None => Ok(value),
        }
    }

    pub fn define_function_from(
        &mut self,
        source: &mut Environment,
        name: Option<&str>,
        value: Value,
    ) -> Result<Value, RuntimeError> {
        match value {
            Value::FunctionReference(ref reference) => {
                let mut function = source.function(reference)?;
                function.set_name(name);
                self.define_function(function)
            }
            constant => self.define_value(name, constant),
        }
    }
}
