//! The Syntax Tree for the tortuga grammar.

mod expression;
mod terminal;
mod traversal;

pub use crate::grammar::traversal::{Iter, PostOrderIterator, PreOrderIterator};
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

    pub fn iter(&self) -> Iter<'a, '_> {
        self.into()
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
        let mut iterator = self.iter().peekable();

        while let Some(node) = iterator.next() {
            let is_last = iterator.peek().map(|n| n.discovered()).unwrap_or(true);

            match node.expression() {
                Expression::Internal(internal) if node.discovered() => {
                    match internal.kind() {
                        InternalKind::Block => f.write_char(']')?,
                        _ => f.write_char(')')?,
                    }

                    if !is_last {
                        f.write_char(' ')?;
                    }
                }
                Expression::Internal(internal) => {
                    match internal.kind() {
                        InternalKind::Block => f.write_char('[')?,
                        _ => f.write_char('(')?,
                    }

                    let kind = internal.kind().to_string();
                    f.write_str(kind.as_str())?;

                    if !kind.is_empty() {
                        f.write_char(' ')?;
                    }
                }
                Expression::Terminal(terminal) if node.discovered() => {
                    write!(f, "{}", terminal)?;

                    if !is_last {
                        f.write_char(' ')?;
                    }
                }
                Expression::Terminal(_) => (),
            }
        }

        Ok(())
    }
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
        let actual: Vec<Expression<'static>> = program
            .iter_post_order()
            .map(|n| n.expression())
            .cloned()
            .collect();

        assert_eq!("(+ 3 2)", program.to_string().as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn grouping() {
        let mut program = Program::default();

        let left = Number::positive("3");
        let left_index = program.insert(left.clone());

        let right = Number::positive("2");
        let right_index = program.insert(right.clone());

        let grouping = Internal::new(InternalKind::Grouping, vec![left_index, right_index]);
        let grouping_index = program.insert(grouping.clone());

        program.mark_root(grouping_index);

        assert_eq!("(3 2)", program.to_string().as_str());
    }

    #[test]
    fn display() {
        let mut program = Program::default();

        let function = Identifier::from("f");
        let function_index = program.insert(function);

        let parameter = Identifier::from("x");
        let parameter_index = program.insert(parameter);

        let declaration = Internal::new(InternalKind::Call, vec![function_index, parameter_index]);
        let declaration_index = program.insert(declaration);

        let left_index = program.insert(parameter);
        let right_index = program.insert(parameter);

        let multiply = Internal::new(InternalKind::Multiply, vec![left_index, right_index]);
        let multiply_index = program.insert(multiply);

        let equality = Internal::new(
            InternalKind::Equality,
            vec![declaration_index, multiply_index],
        );
        let equality_index = program.insert(equality);

        let invocation_index = program.insert(function);
        let argument_index = program.insert(Number::positive("2"));
        let call = Internal::new(InternalKind::Call, vec![invocation_index, argument_index]);
        let call_index = program.insert(call);

        program.mark_root(equality_index);
        program.mark_root(call_index);

        assert_eq!("(= (f x) (* x x)) (f 2)", program.to_string().as_str());
    }
}
