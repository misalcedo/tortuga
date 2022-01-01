//! A Constraint Satisfaction Problem solver.
//! Used to determine the runtime value of a variable.
//!
//! See <https://en.wikipedia.org/wiki/Constraint_programming>

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

    /// Defines a variable as having a given [`Value`].
    /// Returns the previously defined value, if any.
    pub fn define(&mut self, name: &str, value: &Value) -> Option<Value> {
        match self.variables.entry(name.to_string()) {
            Vacant(entry) => {
                entry.insert(*value);
                None
            }
            Occupied(entry) => Some(*entry.get()),
        }
    }
}
