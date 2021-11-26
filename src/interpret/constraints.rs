//! A Constraint Satisfaction Problem solver.
//! Used to determine the runtime value of a variable.
//!
//! See <https://en.wikipedia.org/wiki/Constraint_programming>

use crate::errors::RuntimeError;
use crate::grammar::ComparisonOperator;
use std::collections::HashMap;

/// Solves constraints to determine variable values.
/// Uses a combination of perturbation and refinement to constain variables to a value.
#[derive(Debug, Default)]
pub struct Environment {
    variables: HashMap<String, Variable>,
}

impl Environment {
    pub fn value_of(&self, name: &str) -> Option<f64> {
        let variable = self.variables.get(name)?;
        
        variable.value()
    }

    pub fn refine(
        &mut self,
        name: &str,
        refinement: ComparisonOperator,
        right: f64,
    ) -> Result<f64, RuntimeError> {
        let variable = self.variables.entry(name.to_string()).or_default();

        if variable.refine(refinement, right).is_err() {
            return Err(RuntimeError::InvalidRefinement(
                name.to_string(),
                refinement,
                right,
                variable.domain(),
            ));
        }

        Ok(variable.sample())
    }
}

/// A constraint satisfaction problem is defined by a set of variables, domains, and constrainst.
/// This structure holds the domain, constraints, and value for a given variable.
/// A variable is only present when the variable is fully constrained.
#[derive(Clone, Debug, Default)]
struct Variable {
    domain: Domain,
    constraints: Vec<(ComparisonOperator, f64)>,
    value: Option<f64>,
}

impl Variable {
    fn value(&self) -> Option<f64> {
        self.value
    }

    fn sample(&self) -> f64 {
        match self.value {
            Some(value) => value,
            None => self.domain.sample()
        }
    }

    fn domain(&self) -> (f64, f64) {
        self.domain.clone().into()
    }

    fn refine(&mut self, refinement: ComparisonOperator, right: f64) -> Result<(), ()> {
        if self.domain.refine(refinement, right)? {
            self.constraints.push((refinement, right));
        }

        if refinement == ComparisonOperator::EqualTo && self.value.is_none() {
            self.value.insert(right);
            Ok(())
        } else if refinement == ComparisonOperator::EqualTo {
            Err(())
        } else {
            Ok(())
        }
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

impl From<Domain> for (f64, f64) {
    fn from(domain: Domain) -> Self {
        (domain.start, domain.end)
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

    /// Returns true if the refinement shrunk the domain, false otherwise.
    fn refine(&mut self, refinement: ComparisonOperator, right: f64) -> Result<bool, ()> {
        match refinement {
            ComparisonOperator::LessThan => {
                if self.contains(right) && self.end > right {
                    self.end = right - f64::EPSILON;
                    self.start = self.start.min(self.end);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            ComparisonOperator::LessThanOrEqualTo => {
                if self.contains(right) && self.end >= right {
                    self.end = right;
                    self.start = self.start.min(self.end);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            ComparisonOperator::GreaterThan => {
                if self.contains(right) && self.start < right {
                    self.start = right + f64::EPSILON;
                    self.end = self.end.max(self.start);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            ComparisonOperator::GreaterThanOrEqualTo => {
                if self.contains(right) && self.start <= right {
                    self.start = right;
                    self.end = self.end.max(self.start);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            ComparisonOperator::EqualTo => {
                if self.contains(right) {
                    self.start = right;
                    self.end = right;
                    Ok(true)
                } else {
                    Err(())
                }
            }
            ComparisonOperator::NotEqualTo => {
                if self.contains(right) {
                    Err(())
                } else {
                    Ok(false)
                }
            }
            ComparisonOperator::Comparable => Ok(false),
        }
    }
}
