//! Tortuga has no statements. Every piece of code is an expression that produces a value.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use the order of the [`Internal`] variants to denote each precedence level.

use crate::grammar::{Identifier, Number, Uri};
use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Expression<'a> {
    kind: ExpressionKind<'a>,
    children: Vec<ExpressionReference>,
}

impl<'a> Expression<'a> {
    pub(crate) fn new(kind: ExpressionKind<'a>, children: Vec<ExpressionReference>) -> Self {
        Expression { kind, children }
    }

    pub(crate) fn children(&self) -> &[ExpressionReference] {
        self.children.as_slice()
    }

    pub fn kind(&self) -> &ExpressionKind<'a> {
        &self.kind
    }

    pub fn leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Default for Expression<'_> {
    fn default() -> Self {
        Expression::from(Identifier::from("_"))
    }
}

/// An opaque reference to an [`Expression`] inserted into a [`Program`].
/// Used to refer to an expression as a child of another expression.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExpressionReference(pub(crate) usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ExpressionKind<'a> {
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
    Number(Number<'a>),
    Identifier(Identifier<'a>),
    Uri(Uri<'a>),
}

impl Display for ExpressionKind<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionKind::Block => Ok(()),
            ExpressionKind::Equality => f.write_char('='),
            ExpressionKind::Modulo => f.write_char('%'),
            ExpressionKind::Subtract => f.write_char('-'),
            ExpressionKind::Add => f.write_char('+'),
            ExpressionKind::Divide => f.write_char('/'),
            ExpressionKind::Multiply => f.write_char('*'),
            ExpressionKind::Power => f.write_char('^'),
            ExpressionKind::Call => Ok(()),
            ExpressionKind::Grouping => Ok(()),
            ExpressionKind::Condition => f.write_char('?'),
            ExpressionKind::Inequality => f.write_str("<>"),
            ExpressionKind::LessThan => f.write_char('<'),
            ExpressionKind::GreaterThan => f.write_char('>'),
            ExpressionKind::LessThanOrEqualTo => f.write_str("<="),
            ExpressionKind::GreaterThanOrEqualTo => f.write_str(">="),
            ExpressionKind::Number(n) => write!(f, "{}", n),
            ExpressionKind::Identifier(i) => write!(f, "{}", i),
            ExpressionKind::Uri(u) => write!(f, "{}", u),
        }
    }
}

impl<'a> From<Number<'a>> for Expression<'a> {
    fn from(expression: Number<'a>) -> Self {
        Expression {
            kind: ExpressionKind::Number(expression),
            children: vec![],
        }
    }
}

impl<'a> From<Identifier<'a>> for Expression<'a> {
    fn from(expression: Identifier<'a>) -> Self {
        Expression {
            kind: ExpressionKind::Identifier(expression),
            children: vec![],
        }
    }
}

impl<'a> From<Uri<'a>> for Expression<'a> {
    fn from(expression: Uri<'a>) -> Self {
        Expression {
            kind: ExpressionKind::Uri(expression),
            children: vec![],
        }
    }
}
