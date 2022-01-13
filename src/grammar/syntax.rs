use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Program(pub Vec<Expression>);

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Name = Option<Identifier>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Expression {
    Operation(Box<Operation>),
    Tuple(Box<Tuple>),
    Number(Number),
    FunctionCall(Box<FunctionCall>),
    Identifier(Identifier),
    Interval(Box<Interval>),
    Grouping(Box<Expression>),
    PatternMatch(Box<PatternMatch>),
    Constant(Box<Expression>)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PatternMatch {
    pub pattern: Pattern,
    pub block: Block,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Pattern {
    Function(Function),
    Refinement(Refinement),
    Number(NumberPattern),
    Interval(IntervalPattern),
    Tuple(TuplePattern),
    List(Box<ListPattern>),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TuplePattern(pub Vec<Pattern>);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ListPattern {
    pub head: Pattern,
    pub others: Vec<Pattern>,
    pub rest: Name,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct IntervalPattern(EndpointPattern, EndpointPattern);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum EndpointPattern {
    Inclusive(NumberPattern),
    Exclusive(NumberPattern),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Refinement {
    pub lhs: NumberPattern,
    pub comparator: Comparator,
    pub rhs: Expression,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum NumberPattern {
    Real(Name),
    Natural(Name),
    Parts(Name, Name),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Function {
    pub name: Name,
    pub parameters: Vec<Pattern>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Block {
    pub first: Expression,
    pub rest: Vec<Expression>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FunctionCall {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Tuple(pub Vec<Expression>);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Number {
    pub negative: bool,
    pub value: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Identifier(pub String);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Interval(pub Endpoint, pub Endpoint);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Endpoint {
    Inclusive(Expression),
    Exclusive(Expression),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Operation {
    pub lhs: Expression,
    pub operator: Operator,
    pub rhs: Expression,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulo,
    Tolerance,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Continuation {
    Expression(Expression),
    Comparison(Box<Comparison>),
}

impl Default for Continuation {
    fn default() -> Self {
        Continuation::Expression(Expression::Tuple(Box::new(Tuple::default())))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Comparison {
    pub lhs: Expression,
    pub comparator: Comparator,
    pub rhs: Continuation,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Comparator {
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    Equal,
    NotEqual,
}
