//! The Syntax Tree for the tortuga grammar.

mod expression;
mod terminal;
mod traversal;

use crate::grammar::traversal::{
    PostOrderIterator, PostOrderIteratorWithHeight, PreOrderIterator, PreOrderIteratorWithHeight,
};
pub use expression::{Expression, ExpressionReference, Internal, InternalKind, Terminal};
use std::fmt::{Display, Formatter, Write};
pub use terminal::{Identifier, Number, Uri};

/// An ordered forest of [`Expression`]s.
/// Each [`Expression`] is a tree with any number of children.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program<'a> {
    expressions: Vec<Expression<'a>>,
    roots: Vec<ExpressionReference>,
}

impl<'a> Program<'a> {
    pub fn mark_root(&mut self, index: ExpressionReference) {
        self.roots.push(index);
    }

    pub fn insert<E: Into<Expression<'a>>>(&mut self, expression: E) -> ExpressionReference {
        let index = self.expressions.len();

        self.expressions.push(expression.into());

        ExpressionReference(index)
    }

    pub fn iter(&self) -> PostOrderIterator<'a, '_> {
        self.into()
    }

    pub fn iter_with_height(&self) -> PostOrderIteratorWithHeight<'a, '_> {
        self.into()
    }

    pub fn iter_pre_order(&self) -> PreOrderIterator<'a, '_> {
        self.into()
    }

    pub fn iter_pre_order_with_height(&self) -> PreOrderIteratorWithHeight<'a, '_> {
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
        let mut iterator = self.iter_pre_order_with_height();
        let missing = Expression::default();

        while let Some((depth, expression)) = iterator.next() {
            match expression {
                Expression::Internal(internal) => {
                    format_internal(f, depth, internal, &missing, &mut iterator)?
                }
                Expression::Terminal(terminal) => write!(f, "{}", terminal)?,
            }
        }

        Ok(())
    }
}

fn format_internal<'a>(
    f: &mut Formatter<'_>,
    depth: usize,
    internal: &Internal,
    missing: &Expression<'a>,
    iterator: &mut PreOrderIteratorWithHeight<'a, '_>,
) -> std::fmt::Result {
    write!(f, "({}", internal)?;

    if internal.kind() != &InternalKind::Grouping && internal.kind() != &InternalKind::Call {
        write!(f, " ")?;
    }

    let children = internal.len();

    for i in 1..=children {
        match iterator.next().unwrap_or((depth + 1, missing)) {
            (height, Expression::Internal(child)) => {
                format_internal(f, height, child, missing, iterator)?
            }
            (_, Expression::Terminal(terminal)) => write!(f, "{}", terminal)?,
        }

        if i < children {
            f.write_str(" ")?;
        }
    }

    f.write_char(')')?;

    if depth == 0 {
        f.write_char(' ')?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let actual: Vec<Expression<'static>> = program.iter().cloned().collect();

        assert_eq!(expected, actual);
    }
}
