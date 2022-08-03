use crate::grammar::ExpressionReference;
use crate::parser::SyntacticalError;
use crate::{Parser, TokenKind};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Comparison,
    Term,
    Factor,
    Power,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::Assignment,
            Self::Assignment => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Power,
            Self::Power => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}

type ParseFunction<'a, I, R> =
    fn(&mut Parser<'a, I, R>) -> Result<ExpressionReference, SyntacticalError>;

#[derive(Clone, Copy)]
pub struct ParseRule<'a, I, R> {
    kind: TokenKind,
    prefix: Option<ParseFunction<'a, I, R>>,
    infix: Option<ParseFunction<'a, I, R>>,
    precedence: Precedence,
}

impl<'a, I, R> PartialEq for ParseRule<'a, I, R> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.precedence == other.precedence
    }
}

impl<'a, I, R> PartialOrd for ParseRule<'a, I, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.kind.partial_cmp(&other.kind)
    }
}

impl<'a, I, R> ParseRule<'a, I, R> {
    pub fn new(
        kind: TokenKind,
        prefix: ParseFunction<'a, I, R>,
        infix: ParseFunction<'a, I, R>,
        precedence: Precedence,
    ) -> Self {
        ParseRule {
            kind,
            prefix: Some(prefix),
            infix: Some(infix),
            precedence,
        }
    }

    pub fn new_prefix(kind: TokenKind, prefix: ParseFunction<'a, I, R>) -> Self {
        ParseRule {
            kind,
            prefix: Some(prefix),
            infix: None,
            precedence: Precedence::None,
        }
    }

    pub fn new_infix(
        kind: TokenKind,
        infix: ParseFunction<'a, I, R>,
        precedence: Precedence,
    ) -> Self {
        ParseRule {
            kind,
            prefix: None,
            infix: Some(infix),
            precedence,
        }
    }

    pub fn placeholder(kind: TokenKind) -> Self {
        ParseRule {
            kind,
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }

    pub fn prefix(&self) -> Option<&ParseFunction<'a, I, R>> {
        self.prefix.as_ref()
    }

    pub fn infix(&self) -> Option<&ParseFunction<'a, I, R>> {
        self.infix.as_ref()
    }

    pub fn precedence(&self) -> Precedence {
        self.precedence
    }
}
