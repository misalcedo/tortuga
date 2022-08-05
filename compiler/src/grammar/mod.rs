//! The Syntax Tree for the tortuga grammar.

mod expression;
mod terminal;
mod traversal;

pub use crate::grammar::traversal::{PostOrderIterator, PreOrderIterator};
pub use expression::{Expression, ExpressionReference, Internal, InternalKind, Terminal};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Write};
pub use terminal::{Identifier, Number, Uri};

/// An ordered forest of [`Expression`]s.
/// Each [`Expression`] is a tree with any number of children.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program<'a> {
    expressions: Vec<Expression<'a>>,
    roots: BTreeSet<usize>,
}

impl<'a> Program<'a> {
    pub fn mark_root(&mut self, index: ExpressionReference) {
        self.roots.insert(index.0);
    }

    pub fn insert<E: Into<Expression<'a>>>(&mut self, expression: E) -> ExpressionReference {
        let index = self.expressions.len();

        self.expressions.insert(index, expression.into());

        ExpressionReference(index)
    }

    pub fn iter_post_order(&self) -> PostOrderIterator<'a, '_> {
        self.into()
    }

    pub fn iter_pre_order(&self) -> PreOrderIterator<'a, '_> {
        self.into()
    }

    pub fn len(&self) -> usize {
        self.expressions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.expressions.is_empty()
    }
}

impl Display for Program<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iterator = self.iter_pre_order();
        let missing = Expression::default();

        while let Some((_, expression)) = iterator.next() {
            match expression {
                Expression::Internal(internal) => {
                    format_internal(f, internal, &missing, &mut iterator)?
                }
                Expression::Terminal(terminal) => write!(f, "{}", terminal)?,
            }
        }

        Ok(())
    }
}

static PARENTHESIS_KINDS: &[InternalKind] = &[InternalKind::Call, InternalKind::Grouping];

fn format_internal<'a>(
    f: &mut Formatter<'_>,
    internal: &Internal,
    missing: &Expression<'a>,
    iterator: &mut PreOrderIterator<'a, '_>,
) -> std::fmt::Result {
    write!(f, "({}", internal)?;

    if !PARENTHESIS_KINDS.contains(internal.kind()) {
        write!(f, " ")?;
    }

    let children = internal.len();

    for i in 1..=children {
        match iterator.next().unwrap_or((0, missing)) {
            (_, Expression::Internal(child)) => format_internal(f, child, missing, iterator)?,
            (_, Expression::Terminal(terminal)) => write!(f, "{}", terminal)?,
        }

        if i < children {
            f.write_str(" ")?;
        }
    }

    f.write_char(')')?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::traversal::WithoutScopeDepth;

    #[test]
    fn add() {
        let mut program = Program::default();

        let left = Number::positive("3");
        let left_index = program.insert(left.clone());

        let right = Number::positive("2");
        let right_index = program.insert(right.clone());

        let add = Internal::new(InternalKind::Add, vec![left_index, right_index]);
        let add_index = program.insert(add.clone());

        program.mark_root(add_index);

        let expected: Vec<Expression<'static>> = vec![left.into(), right.into(), add.into()];
        let actual: Vec<Expression<'static>> = program
            .iter_post_order()
            .without_scope_depth()
            .cloned()
            .collect();

        assert_eq!(expected, actual);
    }
}
