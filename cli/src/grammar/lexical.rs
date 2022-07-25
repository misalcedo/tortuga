//! The lexical grammar rules for Tortuga.

/// The name of a function or constant.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a new instance of an [`Identifier`].
    pub fn new(lexeme: &str) -> Self {
        Identifier(lexeme.to_string())
    }

    /// The [`str`] representation of this [`Identifier`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A numerical literal.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Number(String);

impl Number {
    /// Creates a new instance of a [`Number`].
    pub fn new(lexeme: &str) -> Self {
        Number(lexeme.to_string())
    }

    /// The [`str`] representation of this [`Number`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
