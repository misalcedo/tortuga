//! A Constraint Satisfaction Problem solver.
//! Used to determine the runtime value of a variable.
//!
//! See <https://en.wikipedia.org/wiki/Constraint_programming>

use crate::errors::RuntimeError;
use crate::grammar::ComparisonOperator;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

/// The variable context for a single lexical scope.
/// Environments are a tree, the root of the tree has not parent.
#[derive(Debug, Default)]
pub struct Environment {
    parent: Option<usize>,
    variables: HashMap<String, Variable>,
}

impl Environment {
    pub fn value_of(&self, name: &str) -> Option<f64> {
        let variable = self.variables.get(name)?;
        
        Some(variable.0)
    }

    /// Refines a variale as having a given value.
    pub fn refine(
        &mut self,
        name: &str,
        comparator: ComparisonOperator,
        value: f64,
    ) -> Result<f64, RuntimeError> {
        if comparator != ComparisonOperator::EqualTo {
            return Err(RuntimeError::InvalidRefinement(name.to_string(), comparator, value))
        }
        
        match self.variables.entry(name.to_string()) {
            Vacant(entry) => {
                entry.insert(Variable(value));
                Ok(value)
            }
            Occupied(entry) => {
                Err(RuntimeError::AlreadyDefined(name.to_string(), entry.get().0, value))
            }
        }
    }
}

/// A constraint satisfaction problem is defined by a set of variables, domains, and constrainst.
/// This structure holds the domain, constraints, and value for a given variable.
/// A variable is only present when the variable is fully constrained.
#[derive(Copy, Clone, Debug)]
struct Variable(f64);