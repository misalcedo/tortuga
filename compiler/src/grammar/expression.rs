//! Tortuga has no statements. Every piece of code is an expression that produces a value.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use the order of the [`Internal`] variants to denote each precedence level.

use crate::grammar::{Identifier, Number, Uri};
use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Expression<'a> {
    Internal(Internal),
    Terminal(Terminal<'a>),
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Internal(i) => write!(f, "{}", i),
            Expression::Terminal(t) => write!(f, "{}", t),
        }
    }
}

impl Default for Expression<'_> {
    fn default() -> Self {
        Expression::Terminal(Terminal::Identifier(Identifier::from("_")))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Internal {
    kind: InternalKind,
    children: Vec<ExpressionReference>,
}

/// An opaque reference to an [`Expression`] inserted into a [`Program`].
/// Used to refer to an expression as a child of another expression.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExpressionReference(pub(crate) usize);

impl Display for Internal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Internal {
    pub fn new(kind: InternalKind, children: Vec<ExpressionReference>) -> Self {
        Internal { kind, children }
    }

    pub fn kind(&self) -> &InternalKind {
        &self.kind
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub(crate) fn children(&self) -> &[ExpressionReference] {
        self.children.as_slice()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InternalKind {
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
    Inequality,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
}

impl Display for InternalKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalKind::Block => Ok(()),
            InternalKind::Equality => f.write_char('='),
            InternalKind::Modulo => f.write_char('%'),
            InternalKind::Subtract => f.write_char('-'),
            InternalKind::Add => f.write_char('+'),
            InternalKind::Divide => f.write_char('/'),
            InternalKind::Multiply => f.write_char('*'),
            InternalKind::Power => f.write_char('^'),
            InternalKind::Call => Ok(()),
            InternalKind::Grouping => Ok(()),
            InternalKind::Inequality => f.write_str("<>"),
            InternalKind::LessThan => f.write_char('<'),
            InternalKind::GreaterThan => f.write_char('>'),
            InternalKind::LessThanOrEqualTo => f.write_str("<="),
            InternalKind::GreaterThanOrEqualTo => f.write_str(">="),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Terminal<'a> {
    Number(Number<'a>),
    Identifier(Identifier<'a>),
    Uri(Uri<'a>),
}

impl Display for Terminal<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminal::Number(n) => write!(f, "{}", n),
            Terminal::Identifier(i) => write!(f, "{}", i),
            Terminal::Uri(u) => write!(f, "{}", u),
        }
    }
}

impl From<Internal> for Expression<'_> {
    fn from(expression: Internal) -> Self {
        Expression::Internal(expression)
    }
}

impl<'a> From<Number<'a>> for Expression<'a> {
    fn from(expression: Number<'a>) -> Self {
        Expression::Terminal(Terminal::Number(expression))
    }
}

impl<'a> From<Identifier<'a>> for Expression<'a> {
    fn from(expression: Identifier<'a>) -> Self {
        Expression::Terminal(Terminal::Identifier(expression))
    }
}

impl<'a> From<Uri<'a>> for Expression<'a> {
    fn from(expression: Uri<'a>) -> Self {
        Expression::Terminal(Terminal::Uri(expression))
    }
}
