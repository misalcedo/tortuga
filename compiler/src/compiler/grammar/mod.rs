//! The Syntax Tree for the tortuga grammar.

mod expression;
mod terminal;
mod traversal;

pub use crate::compiler::grammar::traversal::{
    Iter, Node, NodeIterator, PostOrderIterator, PreOrderIterator, ReferenceIterator,
};
pub use expression::{Expression, ExpressionKind, ExpressionReference};
use std::fmt::{Display, Formatter};
pub use terminal::{Identifier, Number, Uri};

/// An ordered forest of [`Expression`]s.
/// Each [`Expression`] is a tree with any number of children.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program<'a> {
    expressions: Vec<Expression<'a>>,
    roots: Vec<usize>,
}

impl<'a> Program<'a> {
    pub fn mark_root(&mut self, index: ExpressionReference) {
        self.roots.push(index.0);
    }

    pub fn insert<E: Into<Expression<'a>>>(&mut self, expression: E) -> ExpressionReference {
        let index = self.expressions.len();

        self.expressions.insert(index, expression.into());

        ExpressionReference(index)
    }

    pub fn iter(&self) -> Iter<'a, '_> {
        self.into()
    }

    pub fn roots(&self) -> ReferenceIterator<'a, '_, std::slice::Iter<'_, usize>> {
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
            let expression = node.expression();

            if !node.discovered() && !expression.leaf() {
                let open = match expression.kind() {
                    ExpressionKind::Block => '[',
                    _ => '(',
                };

                write!(f, "{}", open)?;

                let kind = expression.kind().to_string();

                write!(f, "{}", kind)?;

                if !kind.is_empty() {
                    write!(f, " ")?;
                }
            } else if node.discovered() && !expression.leaf() {
                let close = match expression.kind() {
                    ExpressionKind::Block => ']',
                    _ => ')',
                };

                write!(f, "{}", close)?;

                if !is_last {
                    write!(f, " ")?;
                }
            } else if node.discovered() && expression.leaf() {
                write!(f, "{}", expression)?;

                if !is_last {
                    write!(f, " ")?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::grammar::traversal::NodeIterator;

    #[test]
    fn add() {
        let mut program = Program::default();

        let left = Number::positive("3");
        let left_index = program.insert(left.clone());

        let right = Number::positive("2");
        let right_index = program.insert(right.clone());

        let add = Expression::new(ExpressionKind::Add, vec![left_index, right_index]);
        let add_index = program.insert(add.clone());

        program.mark_root(add_index);

        let expected: Vec<Expression<'_>> = vec![left.into(), right.into(), add.into()];
        let actual: Vec<Expression<'_>> = program
            .iter()
            .post_order()
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

        let grouping = Expression::new(ExpressionKind::Grouping, vec![left_index, right_index]);
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

        let declaration =
            Expression::new(ExpressionKind::Call, vec![function_index, parameter_index]);
        let declaration_index = program.insert(declaration);

        let left_index = program.insert(parameter);
        let right_index = program.insert(parameter);

        let multiply = Expression::new(ExpressionKind::Multiply, vec![left_index, right_index]);
        let multiply_index = program.insert(multiply);

        let equality = Expression::new(
            ExpressionKind::Equality,
            vec![declaration_index, multiply_index],
        );
        let equality_index = program.insert(equality);

        let invocation_index = program.insert(function);
        let argument_index = program.insert(Number::positive("2"));
        let call = Expression::new(ExpressionKind::Call, vec![invocation_index, argument_index]);
        let call_index = program.insert(call);

        program.mark_root(equality_index);
        program.mark_root(call_index);

        assert_eq!("(= (f x) (* x x)) (f 2)", program.to_string().as_str());
    }

    #[test]
    fn roots() {
        let mut program = Program::default();

        let function = Identifier::from("f");
        let function_index = program.insert(function);

        let parameter = Identifier::from("x");
        let parameter_index = program.insert(parameter);

        let declaration =
            Expression::new(ExpressionKind::Call, vec![function_index, parameter_index]);
        let declaration_index = program.insert(declaration);

        let left_index = program.insert(parameter);
        let right_index = program.insert(parameter);

        let multiply = Expression::new(ExpressionKind::Multiply, vec![left_index, right_index]);
        let multiply_index = program.insert(multiply);

        let equality = Expression::new(
            ExpressionKind::Equality,
            vec![declaration_index, multiply_index],
        );
        let equality_index = program.insert(equality);

        let invocation_index = program.insert(function);
        let argument_index = program.insert(Number::positive("2"));
        let call = Expression::new(ExpressionKind::Call, vec![invocation_index, argument_index]);
        let call_index = program.insert(call);

        program.mark_root(equality_index);
        program.mark_root(call_index);

        let expected = vec![
            Node::new(&program, equality_index.0).unwrap(),
            Node::new(&program, call_index.0).unwrap(),
        ];
        let nodes: Vec<Node<'_, '_>> = program.roots().collect();

        for n in program.iter() {
            if n.height() == 0 {
                println!("ALL({}): {:?}", n.height(), n.expression().kind())
            }
        }

        for n in program.iter().filter(|n| n.root()) {
            println!("ROOTS({}): {:?}", n.height(), n.expression().kind())
        }

        assert_eq!(expected, nodes);
    }
}