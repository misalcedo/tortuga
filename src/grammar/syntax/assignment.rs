//! Grammar rules for function declarations and pattern matching.

use crate::grammar::lexical::Identifier;
use crate::grammar::syntax::{Arithmetic, Comparator, Expression, List};
use std::fmt::{self, Display, Formatter, Write};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Assignment {
    function: Function,
    block: Block,
}

impl Assignment {
    /// Creates a new `assignment` grammar rule.
    pub fn new(function: Function, block: Block) -> Self {
        Assignment { function, block }
    }

    /// Get the `function` defined by this `Assignment`.
    pub fn function(&self) -> &Function {
        &self.function
    }

    /// Get the code block to be executed on a call to this `Assignment`'s `function`.
    pub fn block(&self) -> &Block {
        &self.block
    }
}

pub type Block = List<Expression>;

#[derive(Clone, Debug, Eq)]
pub enum Pattern {
    Function(Box<Function>),
    Refinement(Box<Refinement>),
    Bounds(Box<Bounds>),
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pattern::Function(a), Pattern::Function(b)) => a.parameters == b.parameters,
            (Pattern::Refinement(a), Pattern::Refinement(b)) => {
                a.comparator == b.comparator && a.constraint == b.constraint
            }
            (Pattern::Bounds(a), Pattern::Bounds(b)) => a.left == b.left && a.right == b.right,
            _ => false,
        }
    }
}

impl Pattern {
    /// The [`Name`] of this [`Pattern`].
    pub fn name(&self) -> &Name {
        match self {
            Pattern::Function(function) => function.name(),
            Pattern::Refinement(refinement) => refinement.name(),
            Pattern::Bounds(bounds) => bounds.name(),
        }
    }
}

impl From<Function> for Pattern {
    fn from(function: Function) -> Self {
        Pattern::Function(Box::new(function))
    }
}

impl From<Refinement> for Pattern {
    fn from(refinement: Refinement) -> Self {
        Pattern::Refinement(Box::new(refinement))
    }
}

impl From<Bounds> for Pattern {
    fn from(bounds: Bounds) -> Self {
        Pattern::Bounds(Box::new(bounds))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    name: Name,
    parameters: Vec<Pattern>,
}

pub type Parameters = Vec<Pattern>;

impl Function {
    /// Create a new instance of a `Function`.
    pub fn new(name: Name, parameters: Vec<Pattern>) -> Self {
        Function { name, parameters }
    }

    /// The `Name` of this `Function`.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The `Parameters` necessary to invoke this `Function`.
    pub fn parameters(&self) -> &[Pattern] {
        self.parameters.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Name {
    Anonymous,
    Identified(Identifier),
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Name::Anonymous => f.write_char('_'),
            Name::Identified(identifier) => f.write_str(identifier.as_str()),
        }
    }
}

impl Name {
    /// The [`str`] representation of this [`Name`].
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Name::Anonymous => None,
            Name::Identified(identifier) => Some(identifier.as_str()),
        }
    }
}

impl From<Identifier> for Name {
    fn from(identifier: Identifier) -> Self {
        Name::Identified(identifier)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bounds {
    left: Bound,
    name: Name,
    right: Bound,
}

impl Bounds {
    /// Create a new `Bounds` pattern.
    pub fn new(left: Bound, name: Name, right: Bound) -> Self {
        Bounds { left, name, right }
    }

    /// The left `Bound` on this `Bounds` pattern.
    pub fn left(&self) -> &Bound {
        &self.left
    }

    /// The `Name` of this `Bounds`.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The right `Bound` on this `Bounds` pattern.
    pub fn right(&self) -> &Bound {
        &self.right
    }
}

/// The singular bound on a `range` pattern.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bound {
    constraint: Arithmetic,
    inequality: Inequality,
}

impl Bound {
    /// Create a new `Bound` pattern.
    pub fn new(constraint: Arithmetic, inequality: Inequality) -> Self {
        Bound {
            constraint,
            inequality,
        }
    }

    /// The constraint this pattern matches.
    pub fn constraint(&self) -> &Arithmetic {
        &self.constraint
    }

    /// The inequality to this pattern's value with.
    pub fn inequality(&self) -> &Inequality {
        &self.inequality
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Inequality {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
}

impl fmt::Display for Inequality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Inequality::LessThan => f.write_char('<'),
            Inequality::LessThanOrEqualTo => f.write_str("<="),
            Inequality::GreaterThan => f.write_char('>'),
            Inequality::GreaterThanOrEqualTo => f.write_str(">="),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refinement {
    name: Name,
    comparator: Comparator,
    constraint: Arithmetic,
}

impl Refinement {
    /// Creates a new instance of a `Refinement`.
    pub fn new(name: Name, comparator: Comparator, constraint: Arithmetic) -> Self {
        Refinement {
            name,
            comparator,
            constraint,
        }
    }

    /// The `Name` defined when this pattern matches.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The comparison operator use by this `Refinement`.
    pub fn comparator(&self) -> &Comparator {
        &self.comparator
    }

    /// The `Arithmetic` value used to constrain the name defined by this `Refinement`.
    pub fn constraint(&self) -> &Arithmetic {
        &self.constraint
    }
}
