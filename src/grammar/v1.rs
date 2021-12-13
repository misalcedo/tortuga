//! The complete grammar definition for Tortuga.

/// program → expression* EOF ;
pub struct Program {
    expressions: Vec<Expression>
}

/// expression → modulo | assignment ;
pub enum Expression {
    Modulo(Modulo),
    Assignment(Assignment)
}

/// modulo     → term ( "%" term )* ;
pub struct Modulo {

}

/// term       → factor ( sign factor )* ;
pub struct Term {
    first: Factor,
    rest: Vec<(Sign, Factor)>
}

/// factor     → exponent ( ( "*" | "/" ) exponent )* ;
pub struct Factor {
    first: Exponent,
    rest: Vec<(FactorOperation, Exponent)>
}

pub enum FactorOperation {
    Multiply,
    Divide
}

/// exponent   → primary ( "^" primary )* ;
pub struct Exponent {
    first: Primary,
    rest: Vec<Primary>
}

/// primary    → number | call | "(" expression ")" ;
pub enum Primary {
    Number(Number),
    Call(Call),
    Grouping(Box<Expression>)
}

/// call       → IDENTIFIER ( "(" arguments ")" )? ;
pub struct Call {
    identifier: Identifier,
    arguments: Option<Arguments>
}

/// number     → sign? NUMBER | NUMBER_WITH_RADIX ;
pub enum Number {
    Radix(String),
    Decimal(Sign, Decimal)
}

/// number     → sign? NUMBER | NUMBER_WITH_RADIX ;
/// DECIMAL                 → DIGIT+ ( "." DIGIT* )? | "." DIGIT+ ;
pub struct Decimal {
    value: String
}

/// assignment → function "=" block ;
pub struct Assignment {
    function: Function,
    block: Block
}

/// block      → expression | "[" expression expression+ "]" ;
pub struct Block {
    first: Box<Expression>,
    rest: Vec<Expression>
}


/// function → name ( "(" parameters ")" )? ;
pub struct Function {
    name: Name,
    parameters: Option<Parameters>
}

/// name     → "_" | IDENTIFIER ;
pub struct Name {
    identifier: Option<Identifier>
}


/// IDENTIFIER              → \{alphabetic} ( ( "_" | \{alphanumeric} )*  \{alphanumeric} )? ;
pub struct Identifier {
    value: String
}

/// parameters → pattern ( "," pattern )* ;
pub struct Parameters {
    first: Pattern,
    rest: Vec<Pattern>
}

/// pattern  → function | range | identity ;
pub enum Pattern {
    Function(Box<Function>),
    Range(Range),
    Identity(Identity)
}

/// range    → ( expression lesser )? name ( greater expression )? ;
pub struct Range {
    left: Option<CompareLeft>,
    name: Identifier,
    right: Option<CompareRight>
}

pub struct CompareLeft {
    expression: Box<Expression>,
    comparison: Lesser
}

pub struct CompareRight {
    expression: Box<Expression>,
    comparison: Greater,
}

/// sign     → "+" | "-" ;
pub enum Sign {
    Plus,
    Minus
}

/// lesser   → "<" | "<=" ;
pub enum Lesser {
    LessThan,
    LessThanOrEqualTo,
}

/// greater  → ">" | ">=" ;
pub enum Greater {
    GreaterThan,
    GreaterThanOrEqualTo,
}

/// expression | name equality expression | expression equality name
pub struct Identity {
    name: Option<Identifier>,
    expression: Box<Expression>,
    equality: Equality,
}

/// equality → "=" | "<>" ;
pub enum Equality {
    EqualTo,
    NotEqualTo
}

/// arguments  → expression ( "," expression )* ;
pub struct Arguments {
    first: Expression,
    rest: Vec<Expression>
} 
