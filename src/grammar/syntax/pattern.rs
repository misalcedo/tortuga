//! Grammar rules for pattern matching within function declarations.

use crate::grammar::lexical::Identifier;
use crate::grammar::syntax::expression::Number;
use crate::grammar::syntax::List;

/// pattern  → function | range | identity ;
pub enum Pattern {
    Function(Box<Function>),
    Range(Range),
    Identity(Identity),
}

/// function → name ( "(" parameters ")" )? ;
pub struct Function {
    name: Name,
    parameters: Option<Parameters>,
}

impl Function {
    /// Create a new instance of a `Function`.
    pub fn new(name: Name, parameters: Option<Parameters>) -> Self {
        Function { name, parameters }
    }

    /// The `Name` of this `Function`.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The `Parameters` necessary to invoke this `Function`.
    pub fn parameters(&self) -> Option<&Parameters> {
        self.parameters.as_ref()
    }
}

/// parameters → pattern ( "," pattern )* ;
pub type Parameters = List<Pattern>;

/// name → "_" | IDENTIFIER ;
pub enum Name {
    Anonymous,
    Identified(Identifier),
}

/// range → number inequality name | ( number inequality )? name inequality number ;
pub enum Range {
    Left(Bound),
    Both(Bounds),
}

/// The singular bound on a `range` pattern.
pub struct Bound {
    value: Number,
    inequality: Inequality,
}

impl Bound {
    /// Create a new `Bound` pattern.
    pub fn new(value: Number, inequality: Inequality) -> Self {
        Bound { value, inequality }
    }

    /// The value this pattern matches.
    pub fn value(&self) -> &Number {
        &self.value
    }

    /// The inequality to this pattern's value with.
    pub fn inequality(&self) -> &Inequality {
        &self.inequality
    }
}

/// The bounds on a `range` pattern.
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

    /// The left bound on this `Range` pattern.
    pub fn left(&self) -> &Bound {
        &self.left
    }

    /// The `Name` of this `Bounds`.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The right bound on this `Range` pattern.
    pub fn right(&self) -> &Bound {
        &self.right
    }
}

/// inequality → "<" | "<=" | ">" | ">=" ;
pub enum Inequality {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
}

/// identity → number | name equality number | number equality name ;
pub struct Identity {
    value: Number,
    name: Option<Name>,
}

impl Identity {
    /// Creates a new instance of an `Identity`.
    pub fn new(value: Number, name: Option<Name>) -> Self {
        Identity { value, name }
    }

    /// The value this pattern matches.
    pub fn value(&self) -> &Number {
        &self.value
    }

    /// The `Name` defined when this pattern matches.
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }
}
