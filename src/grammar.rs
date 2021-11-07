//! The Syntax Tree for the tortuga grammar.

/// A statement in the tortuga grammar.
pub enum Statement {
    /// ( Statement )
    Grouping(Grouping),
    /// NUMBER
    Number(Number),
    /// TEXT_REFERENCE
    TextReference(TextReference),
    BinaryOperation(BinaryOperation),
    ComparisonOperation(ComparisonOperation),
}

/// Groups a statement to change the order of precedence.
pub struct Grouping {
    statement: Box<Statement>,
}

/// A numeric literal.
/// Numeric literals represent whole and fractional numbers.
pub struct Number {
    positive: bool,
    whole: u128,
    fractional: u128
}

/// A reference to internationalized text.
/// Text References are opaque types and thus cannot be manipulated in any way.
/// However, the contents of the reference must be valid UTF-8 text.
/// 
/// In order to support runtime switching of translations, external text references can be
/// mapped to languages, countrys, and locales in a fallible manner (i.e. can fail).
pub struct TextReference {
    reference: String
}

/// A language and country pair used to determine which text reference translation to use.
pub struct Locale {
    language: Language,
    country: Country
}

/// A 2-letter language code used to denote which translation of a text reference to use.
/// 
/// See <https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes>
pub enum Language {
    English,
    Spanish,
    Japanese
}

/// A 2-letter country code used to denote which tranlation of a text reference to use.
/// 
/// See <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
pub enum Country {
    UnitedStates
}

/// An operator that takes 2 arguments.
pub struct BinaryOperation {
    left: Box<Statement>,
    operator: Operator,
    right: Box<Statement>
}

pub enum Operator {
    Multiply,
    Divide,
    Plus,
    Minus,
}

/// An operation that compares 2 arguments relative to each other.
pub struct ComparisonOperation {
    left: Box<Statement>,
    comparator: ComparisonOperator,
    right: Box<Statement>
}

/// An operator to compare two items to each other.
pub enum ComparisonOperator {
    LessThan,
    GreaterThan,
    EqualTo,
}
