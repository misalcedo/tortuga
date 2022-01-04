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
    names: HashMap<String, usize>,
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
    pub fn function_reference(&self, name: &str) -> Result<FunctionReference, RuntimeError> {
        let index = self
            .names
            .get(name)
            .ok_or_else(|| RuntimeError::FunctionNotDefined(name.to_string()))?;

        Ok(FunctionReference(*index))
    }

    /// Defines a [`Function`] as having a given name.
    /// Returns the previously defined value as an [`Err`], if any.
    pub fn define_function(
        &mut self,
        function: Function,
    ) -> Result<FunctionReference, RuntimeError> {
        match function.name() {
            Some(name) => match self.names.entry(name.to_string()) {
                Vacant(entry) => {
                    let reference = insert_function(&mut self.functions, function);

                    entry.insert(reference.0);

                    Ok(reference)
                }
                Occupied(entry) => {
                    let index = *entry.get();
                    let existing = self
                        .functions
                        .get_mut(index)
                        .ok_or_else(|| RuntimeError::FunctionNotDefined(name.to_string()))?;

                    existing.merge(function)?;

                    Ok(FunctionReference(index))
                }
            },
            None => Ok(insert_function(&mut self.functions, function)),
        }
    }

    pub fn define_function_from(
        &mut self,
        source: &mut Environment,
        name: Option<&str>,
        value: Value,
    ) -> Result<FunctionReference, RuntimeError> {
        let function = match value {
            Value::FunctionReference(ref reference) => source.function(reference)?,
            constant => Function::new_constant(name, constant, self),
        };

        self.define_function(function)
    }
}

fn insert_function(functions: &mut Vec<Function>, function: Function) -> FunctionReference {
    let index = functions.len();

    functions.push(function);

    FunctionReference(index)
}
