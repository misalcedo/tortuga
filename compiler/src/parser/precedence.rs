use crate::grammar::ExpressionReference;
use crate::parser::SyntacticalError;
use crate::{Parser, TokenKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn next(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Assignment => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Equality,
            Self::Equality => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call => Self::Primary,
            Self::Primary => Self::None,
        }
    }
}

type ParseFunction<I, R> =
    fn(&mut Parser<'_, I, R>) -> Result<ExpressionReference, SyntacticalError>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParseRule<I, R> {
    kind: TokenKind,
    prefix: Option<ParseFunction<I, R>>,
    infix: Option<ParseFunction<I, R>>,
    precedence: Precedence,
}

impl<I, R> ParseRule<I, R> {
    pub fn any(
        kind: TokenKind,
        prefix: ParseFunction<I, R>,
        infix: ParseFunction<I, R>,
        precedence: Precedence,
    ) -> Self {
        ParseRule {
            kind,
            prefix: Some(prefix),
            infix: Some(infix),
            precedence,
        }
    }

    pub fn prefix(kind: TokenKind, prefix: ParseFunction<I, R>) -> Self {
        ParseRule {
            kind,
            prefix: Some(prefix),
            infix: None,
            precedence: Precedence::None,
        }
    }

    pub fn infix(kind: TokenKind, infix: ParseFunction<I, R>, precedence: Precedence) -> Self {
        ParseRule {
            kind,
            prefix: None,
            infix: Some(infix),
            precedence,
        }
    }

    pub fn empty(kind: TokenKind) -> Self {
        ParseRule {
            kind,
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }

    pub fn precedence(&self) -> Precedence {
        self.precedence
    }
}
