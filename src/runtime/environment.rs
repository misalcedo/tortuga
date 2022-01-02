//! A Constraint Satisfaction Problem solver.
//! Used to determine the runtime value of a variable.
//!
//! See <https://en.wikipedia.org/wiki/Constraint_programming>

use crate::grammar::Assignment;
use crate::runtime::Value;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

/// The variable context for a single lexical scope.
/// Environments are a tree, the root of the tree has no parent.
/// Since all variables are immutable and variables are not allowed to shadow each other,
/// environments start as a clone of their parent.
#[derive(Clone, Debug, Default)]
pub struct Environment {
    variables: HashMap<String, Value>,
    functions: Vec<Assignment>,
}

impl Environment {
    /// Creates a child [`Environment`].
    pub fn new_child(&self) -> Environment {
        self.clone()
    }

    /// Get the [`Value`] of the variable with the given [`Identifier`].
    pub fn value(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    /// Get the [`Assignment`] of the variable with the given index.
    pub fn function(&self, reference: usize) -> Option<&Assignment> {
        self.functions.get(reference)
    }

    /// Defines a variable as having a given [`Value`].
    /// Returns the previously defined value, if any.
    pub fn define_value(&mut self, name: &str, value: &Value) -> Option<Value> {
        match self.variables.entry(name.to_string()) {
            Vacant(entry) => {
                entry.insert(*value);
                None
            }
            Occupied(entry) => Some(*entry.get()),
        }
    }

    /// Defines a variable as having a given function.
    /// Returns the previously defined value as an [`Err`], if any.
    pub fn define_function(&mut self, name: &str, function: &Assignment) -> Result<Value, Value> {
        match self.variables.entry(name.to_string()) {
            Vacant(entry) => {
                let index = self.functions.len();

                self.functions.push(function.clone());
                entry.insert(index.into());

                Ok(index.into())
            }
            Occupied(entry) => Err(*entry.get()),
        }
    }
}
