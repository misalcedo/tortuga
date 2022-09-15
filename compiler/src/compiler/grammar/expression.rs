//! Tortuga has no statements. Every piece of code is an expression that produces a value.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use the order of the [`ExpressionKind`] variants to denote each precedence level.

use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Expression<'a> {
    kind: ExpressionKind,
    lexeme: &'a str,
}

impl<'a> Expression<'a> {
    pub fn new(kind: ExpressionKind, lexeme: &'a str) -> Self {
        Expression { kind, lexeme }
    }

    pub fn kind(&self) -> &ExpressionKind {
        &self.kind
    }

    pub fn as_str(&self) -> &'a str {
        self.lexeme
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ExpressionKind::Block => Ok(()),
            ExpressionKind::Call => Ok(()),
            ExpressionKind::Grouping => Ok(()),
            ExpressionKind::Equality => f.write_char('='),
            ExpressionKind::Modulo => f.write_char('%'),
            ExpressionKind::Subtract => f.write_char('-'),
            ExpressionKind::Add => f.write_char('+'),
            ExpressionKind::Divide => f.write_char('/'),
            ExpressionKind::Multiply => f.write_char('*'),
            ExpressionKind::Power => f.write_char('^'),
            ExpressionKind::Condition => f.write_char('?'),
            ExpressionKind::Inequality => f.write_str("<>"),
            ExpressionKind::LessThan => f.write_char('<'),
            ExpressionKind::GreaterThan => f.write_char('>'),
            ExpressionKind::LessThanOrEqualTo => f.write_str("<="),
            ExpressionKind::GreaterThanOrEqualTo => f.write_str(">="),
            ExpressionKind::Number => f.write_str(self.lexeme),
            ExpressionKind::Identifier => f.write_str(self.lexeme),
            ExpressionKind::Uri => f.write_str(self.lexeme),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ExpressionKind {
    Block,
    Equality,
    Modulo,
    Subtract,
    Add,
    Divide,
    Multiply,
    Power,
    Call,
    Grouping,
    Condition,
    Inequality,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    Number,
    Identifier,
    Uri,
}
