use crate::grammar::syntax::{Expression, List};

/// A pair of a comparison operator and the right-hand side expression to compare against.
pub struct Comparison(Operator, Expression);

impl Comparison {
    /// Creates a new operator and right-hand side pair for the comparison rule.
    pub fn new(operator: Operator, rhs: Expression) -> Self {
        Comparison(operator, rhs)
    }
}

/// program → expression ( comparison expression )+ EOF ;
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
pub enum Operator {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,
    Comparable,
}
