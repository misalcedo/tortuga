//! Tortuga has no statements. Every piece of code is an expression that produces a value.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use the order of the [`Internal`] variants to make each precedence level explicit.

use crate::grammar::{Identifier, Number, Uri};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Expression<'a> {
    Internal(Internal),
    Terminal(Terminal<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Internal {
    kind: InternalKind,
    children: Vec<usize>,
}

impl Internal {
    pub fn new(kind: InternalKind, children: Vec<usize>) -> Self {
        Internal { kind, children }
    }

    pub fn kind(&self) -> &InternalKind {
        &self.kind
    }

    pub(crate) fn children(&self) -> &[usize] {
        self.children.as_slice()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InternalKind {
    Assignment,
    Modulo,
    Subtract,
    Add,
    Division,
    Multiply,
    Power,
    Call,
    Grouping,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Terminal<'a> {
    Number(Number<'a>),
    Identifier(Identifier<'a>),
    Uri(Uri<'a>),
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
