use crate::collections::Tree;
use crate::compiler::parse::SyntaxError;
use crate::compiler::{Location, Parser};
use crate::grammar::Expression;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Precedence {
    #[default]
    None,
    Comparison,
    Modulo,
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
            Self::None => Self::Comparison,
            Self::Comparison => Self::Modulo,
            Self::Modulo => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Power,
            Self::Power => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}

pub type PrefixParseFunction<'a, I, R> =
    fn(&mut Parser<'a, I, R>) -> Result<Tree<Expression<'a>>, SyntaxError>;
pub type InfixParseFunction<'a, I, R> = fn(
    &mut Parser<'a, I, R>,
    Location,
    Tree<Expression<'a>>,
) -> Result<Tree<Expression<'a>>, SyntaxError>;

#[derive(Clone, Copy)]
pub struct ParseRule<'a, I, R> {
    prefix: Option<PrefixParseFunction<'a, I, R>>,
    infix: Option<InfixParseFunction<'a, I, R>>,
    precedence: Precedence,
}

impl<'a, I, R> ParseRule<'a, I, R> {
    pub fn new(
        prefix: PrefixParseFunction<'a, I, R>,
        infix: InfixParseFunction<'a, I, R>,
        precedence: Precedence,
    ) -> Self {
        ParseRule {
            prefix: Some(prefix),
            infix: Some(infix),
            precedence,
        }
    }

    pub fn new_prefix(prefix: PrefixParseFunction<'a, I, R>) -> Self {
        ParseRule {
            prefix: Some(prefix),
            infix: None,
            precedence: Precedence::None,
        }
    }

    pub fn new_infix(infix: InfixParseFunction<'a, I, R>, precedence: Precedence) -> Self {
        ParseRule {
            prefix: None,
            infix: Some(infix),
            precedence,
        }
    }

    pub fn prefix(&self) -> Option<&PrefixParseFunction<'a, I, R>> {
        self.prefix.as_ref()
    }

    pub fn infix(&self) -> Option<&InfixParseFunction<'a, I, R>> {
        self.infix.as_ref()
    }

    pub fn precedence(&self) -> Precedence {
        self.precedence
    }
}
