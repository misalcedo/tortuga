use crate::grammar::syntax::{Expression, List};

/// A pair of a comparison operator and the right-hand side expression to compare against.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Comparison(Comparator, Expression);

impl Comparison {
    /// Creates a new operator and right-hand side pair for the comparison rule.
    pub fn new(operator: Comparator, rhs: Expression) -> Self {
        Comparison(operator, rhs)
    }
}

/// program → expression ( comparison expression )+ EOF ;
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Comparator {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,
    Comparable,
}
