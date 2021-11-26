//! A Constraint Satisfaction Problem solver.
//! Used to determine the runtime value of a variable.
//!
//! See <https://en.wikipedia.org/wiki/Constraint_programming>

use crate::errors::RuntimeError;
use crate::grammar::{ComparisonOperation, ComparisonOperator};
use std::collections::HashMap;

/// Solves constraints to determine variable values.
/// Uses a combination of perturbation and refinement to constain variables to a value.
#[derive(Debug, Default)]
pub struct ConstraintSolver {
    variables: HashMap<String, Variable>,
}

impl ConstraintSolver {
    pub fn value_of(&self, variable: &str) -> f64 {
        match self.variables.get(variable) {
            Some(variable) => variable.value(),
            None => Variable::default().value(),
        }
    }
}

/// A constraint satisfaction problem is defined by a set of variables, domains, and constrainst.
/// This structure holds the domain, constraints, and value for a given variable.
/// A variable is only present when the variable is fully constrained.
#[derive(Clone, Debug, Default)]
struct Variable {
    domain: Domain,
    constraints: Vec<ComparisonOperation>,
    value: Option<f64>,
}

impl Variable {
    fn value(&self) -> f64 {
        match self.value {
            Some(value) => value,
            None => self.domain.sample(),
        }
    }

    fn refine(
        &mut self,
        constraint: &ComparisonOperation,
        solver: &ConstraintSolver,
    ) -> Result<(), RuntimeError> {
        self.domain.refine(constraint, solver)?;
        self.constraints.push(constraint.clone());

        Ok(())
    }
}

/// A domain for variables in the real number space.
/// The domain is represented as a range that is inclusive on both ends.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Domain {
    start: f64,
    end: f64,
}

impl Default for Domain {
    fn default() -> Self {
        Domain {
            start: f64::NEG_INFINITY,
            end: f64::INFINITY,
        }
    }
}

impl Domain {
    fn sample(&self) -> f64 {
        if self.contains(1.0) {
            1.0
        } else {
            (self.end + self.start) / 2.0
        }
    }

    fn contains(&self, value: f64) -> bool {
        self.start <= value && value <= self.end
    }

    fn refine(
        &mut self,
        constraint: &ComparisonOperation,
        solver: &ConstraintSolver,
    ) -> Result<(), RuntimeError> {
        Ok(())
    }
}
