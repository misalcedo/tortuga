//! The Syntax Tree for the tortuga grammar.

mod expression;

use crate::collections::forest::RootsIterator;
use crate::collections::tree::{Iter, Node};
use crate::collections::{Forest, Tree};
pub use expression::{Expression, ExpressionKind};
use std::fmt::{Display, Formatter};

/// An ordered forest of [`Expression`]s.
/// Each [`Expression`] is a tree with any number of children.
#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxTree<'a> {
    source: &'a str,
    forest: Forest<(), Expression<'a>>,
}

impl<'a> SyntaxTree<'a> {
    pub fn new(source: &'a str) -> Self {
        SyntaxTree {
            source,
            forest: Forest::from(()),
        }
    }

    pub fn insert<T>(&mut self, expression: T) -> &mut Tree<Expression<'a>>
    where
        T: Into<Tree<Expression<'a>>>,
    {
        self.forest.insert(expression)
    }

    pub fn iter(&self) -> Iter<'_, Expression<'a>> {
        self.forest.iter()
    }

    pub fn roots(&self) -> RootsIterator<'_, Expression<'a>> {
        self.forest.trees()
    }

    pub fn len(&self) -> usize {
        self.forest.len()
    }

    pub fn is_empty(&self) -> bool {
        self.forest.is_empty()
    }

    pub fn as_str(&self) -> &'a str {
        self.source
    }
}

impl Display for SyntaxTree<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iterator = self.roots().peekable();

        while let Some(node) = iterator.next() {
            write!(f, "{}", node)?;

            if iterator.peek().is_some() {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}

impl<'a> Display for Node<'a, Expression<'a>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.data().kind() {
            _ if self.leaf() => write!(f, "{}", self.data())?,
            ExpressionKind::Block => {
                write!(f, "[")?;

                let mut children = self.children().peekable();

                while let Some(child) = children.next() {
                    write!(f, "{}", child)?;

                    if children.peek().is_some() {
                        write!(f, "; ")?;
                    }
                }

                write!(f, "]")?;
            }
            ExpressionKind::Grouping if self.children().len() == 1 => {
                if let Some(child) = self.children().next() {
                    write!(f, "{}", child)?;
                }
            }
            _ => {
                write!(f, "(")?;

                let kind = self.data().to_string();
                if !kind.is_empty() {
                    write!(f, "{} ", kind)?;
                }

                let mut children = self.children().peekable();

                while let Some(child) = children.next() {
                    write!(f, "{}", child)?;

                    if children.peek().is_some() {
                        write!(f, " ")?;
                    }
                }

                write!(f, ")")?;
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
        let source = "3 + 2";
        let mut program = SyntaxTree::new(source);
        let add = program.insert(Expression::new(ExpressionKind::Add, source));

        let left = add.insert(Expression::new(ExpressionKind::Number, "3"));
        let right = add.insert(Expression::new(ExpressionKind::Number, "2"));

        assert_eq!("(+ 3 2)", program.to_string().as_str());
    }

    #[test]
    fn grouping() {
        let source = "(3 + 2)";
        let mut program = SyntaxTree::new(source);

        let grouping = program.insert(Expression::new(ExpressionKind::Grouping, source));

        let left = grouping.insert(Expression::new(ExpressionKind::Number, "3"));
        let right = grouping.insert(Expression::new(ExpressionKind::Number, "2"));

        assert_eq!("(3 2)", program.to_string().as_str());
    }

    #[test]
    fn equality() {
        let mut program = SyntaxTree::new("f(x) = x * x\nf(2)");

        let assignment = program.insert(Expression::new(ExpressionKind::Equality, "f(x) = x * x"));
        let declaration = assignment.insert(Expression::new(ExpressionKind::Call, "f(x)"));
        let function = declaration.insert(Expression::new(ExpressionKind::Identifier, "f"));
        let parameters = declaration.insert(Expression::new(ExpressionKind::Grouping, "(x)"));
        let parameter = parameters.insert(Expression::new(ExpressionKind::Identifier, "x"));

        let multiply = assignment.insert(Expression::new(ExpressionKind::Multiply, "x * x"));
        let lhs = multiply.insert(Expression::new(ExpressionKind::Identifier, "x"));
        let rhs = multiply.insert(Expression::new(ExpressionKind::Identifier, "x"));

        let invocation = program.insert(Expression::new(ExpressionKind::Call, "f(2)"));
        let callee = invocation.insert(Expression::new(ExpressionKind::Identifier, "f"));
        let arguments = invocation.insert(Expression::new(ExpressionKind::Grouping, "(2)"));
        let argument = arguments.insert(Expression::new(ExpressionKind::Number, "2"));

        assert_eq!("(= (f x) (* x x)) (f 2)", program.to_string().as_str());
        assert_eq!(
            program.roots().map(|n| *n.data()).collect::<Vec<_>>(),
            vec![
                Expression::new(ExpressionKind::Equality, "f(x) = x * x"),
                Expression::new(ExpressionKind::Call, "f(2)"),
            ]
        )
    }
}
