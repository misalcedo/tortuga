use crate::grammar::Expression;

/// A pair of a comparison operator and the right-hand side expression to compare against.
pub type Comparison = (Operator, Expression);

/// program → expression ( comparison expression )+ EOF ;
pub struct Comparisons {
    lhs: Expression,
    first: Comparison,
    rest: Comparison,
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
