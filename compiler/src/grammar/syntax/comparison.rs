use crate::grammar::syntax::{Expression, List};
use std::fmt::{self, Write};

/// A pair of a comparison operator and the right-hand side expression to compare against.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comparison(Comparator, Expression);

impl Comparison {
    /// Creates a new operator and right-hand side pair for the comparison rule.
    pub fn new(operator: Comparator, rhs: Expression) -> Self {
        Comparison(operator, rhs)
    }

    /// The comparison operator to compare the left and right -hand sides.
    pub fn comparator(&self) -> &Comparator {
        &self.0
    }

    /// The right-hand side expression of a comparison grammar rule.
    pub fn rhs(&self) -> &Expression {
        &self.1
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comparisons {
    lhs: Expression,
    comparison: List<Comparison>,
}

impl Comparisons {
    /// Create a new `Comparisons` instance.
    pub fn new(lhs: Expression, comparison: List<Comparison>) -> Self {
        Comparisons { lhs, comparison }
    }

    /// The left-hand side of this sequence of `Comparisons`.
    pub fn lhs(&self) -> &Expression {
        &self.lhs
    }

    /// The list of comparison operations to perform.
    pub fn comparisons(&self) -> &List<Comparison> {
        &self.comparison
    }
}

/// Comparison operators.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Comparator {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comparator::LessThan => f.write_char('<'),
            Comparator::LessThanOrEqualTo => f.write_str("<="),
            Comparator::GreaterThan => f.write_char('>'),
            Comparator::GreaterThanOrEqualTo => f.write_str(">="),
            Comparator::EqualTo => f.write_char('='),
            Comparator::NotEqualTo => f.write_str("<>"),
        }
    }
}
