//! The Syntax Tree for the tortuga grammar.

mod number;

pub use number::{Fraction, Number, Sign, DECIMAL_RADIX, MAX_RADIX};
use std::fmt;

/// A list of zero or more expressions in the tortuga grammar.
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    expressions: Vec<Expression>,
}

impl From<Vec<Expression>> for Program {
    fn from(expressions: Vec<Expression>) -> Self {
        Program { expressions }
    }
}

impl Program {
    pub fn expressions(&self) -> &[Expression] {
        self.expressions.as_slice()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program with {} expressions", self.expressions.len())
    }
}

/// A list of zero or more expressions in the tortuga grammar that defines a lexical scope.
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    expressions: Vec<Expression>,
}

impl Block {
    /// Creates a new instance of a block.
    pub fn new(expressions: Vec<Expression>) -> Self {
        Block { expressions }
    }

    pub fn expressions(&self) -> &[Expression] {
        self.expressions.as_slice()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block with {} expressions", self.expressions.len())
    }
}

/// An expression in the tortuga grammar.
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Grouping(Grouping),
    Number(Number),
    Variable(Variable),
    BinaryOperation(BinaryOperation),
    ComparisonOperation(ComparisonOperation),
    Block(Block),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Grouping(grouping) => write!(f, "{}", grouping),
            Self::Number(number) => write!(f, "{}", number),
            Self::Variable(variable) => write!(f, "{}", variable),
            Self::BinaryOperation(operation) => write!(f, "{}", operation),
            Self::ComparisonOperation(operation) => write!(f, "{}", operation),
            Self::Block(block) => write!(f, "{}", block),
        }
    }
}

impl From<Grouping> for Expression {
    fn from(grouping: Grouping) -> Self {
        Expression::Grouping(grouping)
    }
}

impl From<Number> for Expression {
    fn from(number: Number) -> Self {
        Expression::Number(number)
    }
}

impl From<Variable> for Expression {
    fn from(variable: Variable) -> Self {
        Expression::Variable(variable)
    }
}

impl From<BinaryOperation> for Expression {
    fn from(operation: BinaryOperation) -> Self {
        Expression::BinaryOperation(operation)
    }
}

impl From<ComparisonOperation> for Expression {
    fn from(operation: ComparisonOperation) -> Self {
        Expression::ComparisonOperation(operation)
    }
}

impl From<Block> for Expression {
    fn from(block: Block) -> Self {
        Expression::Block(block)
    }
}

/// Groups an expression to change the order of precedence.
#[derive(Clone, Debug, PartialEq)]
pub struct Grouping {
    expression: Box<Expression>,
}

impl Grouping {
    pub fn new(expression: Expression) -> Self {
        Grouping {
            expression: Box::new(expression),
        }
    }

    /// The inner expression of this grouping
    pub fn inner(&self) -> &Expression {
        &self.expression
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.expression)
    }
}

/// An operator that takes 2 arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOperation {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>,
}

impl BinaryOperation {
    pub fn new(left: Expression, operator: Operator, right: Expression) -> Self {
        BinaryOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn operator(&self) -> Operator {
        self.operator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Exponent,
    Multiply,
    Divide,
    Add,
    Subtract,
    Modulo,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Exponent => write!(f, "^"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Modulo => write!(f, "%"),
        }
    }
}

/// An operation that compares 2 arguments relative to each other.
#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonOperation {
    left: Box<Expression>,
    comparator: ComparisonOperator,
    right: Box<Expression>,
}

impl ComparisonOperation {
    pub fn new(left: Expression, comparator: ComparisonOperator, right: Expression) -> Self {
        ComparisonOperation {
            left: Box::new(left),
            comparator,
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn comparator(&self) -> ComparisonOperator {
        self.comparator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl fmt::Display for ComparisonOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.comparator, self.left, self.right)

        // let mut iterator = self.comparisons.iter().peekable();

        // write!(f, "(and ")?;

        // while let Some(kind) = iterator.next() {
        //     write!(f, "{}", kind)?;

        //     if iterator.peek().is_some() {
        //         write!(f, " ")?;
        //     }
        // }

        // write!(f, ")")
    }
}

/// An operator to compare two items to each other.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,
    Comparable,
}

impl ComparisonOperator {
    /// The corresponding operator when swapping the left and right sides of a comparison operation.
    pub fn flip(&self) -> ComparisonOperator {
        match self {
            Self::LessThan => Self::GreaterThan,
            Self::LessThanOrEqualTo => Self::GreaterThanOrEqualTo,
            Self::GreaterThan => Self::LessThan,
            Self::GreaterThanOrEqualTo => Self::LessThanOrEqualTo,
            Self::EqualTo => Self::EqualTo,
            Self::NotEqualTo => Self::NotEqualTo,
            Self::Comparable => Self::Comparable,
        }
    }
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LessThan => write!(f, "<",),
            Self::LessThanOrEqualTo => write!(f, "<=",),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterThanOrEqualTo => write!(f, ">="),
            Self::EqualTo => write!(f, "="),
            Self::NotEqualTo => write!(f, "<>"),
            Self::Comparable => write!(f, "<=>"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: &str) -> Self {
        Variable {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
