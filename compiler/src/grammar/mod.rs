//! The Syntax Tree for the tortuga grammar.

mod expression;
mod terminal;

pub use expression::{Expression, ExpressionReference, Internal, InternalKind, Terminal};
use std::fmt::{Display, Formatter, Write};
pub use terminal::{Identifier, Number, Uri};

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
        PostOrderIterator {
            program: self,
            stack: self.roots.iter().rev().map(|i| (i.0, false)).collect(),
        }
    }

    pub fn iter_pre_order(&self) -> PreOrderIterator<'a, '_> {
        PreOrderIterator {
            program: self,
            stack: self.roots.iter().map(|r| r.0).collect(),
        }
    }
}

impl Display for Program<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iterator = self.iter_pre_order();
        let missing = Expression::default();

        while let Some(expression) = iterator.next() {
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

fn format_internal<'a>(
    f: &mut Formatter<'_>,
    internal: &Internal,
    missing: &Expression<'a>,
    iterator: &mut PreOrderIterator<'a, '_>,
) -> std::fmt::Result {
    write!(f, "({} ", internal)?;

    let children = internal.len();

    for i in 1..=children {
        write!(f, "{}", iterator.next().unwrap_or(missing))?;

        if i < children {
            f.write_str(" ")?;
        }
    }

    f.write_char(')')
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreOrderIterator<'a, 'b> {
    program: &'b Program<'a>,
    stack: Vec<usize>,
}

impl<'a, 'b> Iterator for PreOrderIterator<'a, 'b> {
    type Item = &'b Expression<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;
        let expression = self.program.expressions.get(index)?;

        if let Expression::Internal(internal) = expression {
            for child in internal.children() {
                self.stack.push(child.0);
            }
        }

        Some(expression)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PostOrderIterator<'a, 'b> {
    program: &'b Program<'a>,
    stack: Vec<(usize, bool)>,
}

impl<'a, 'b> Iterator for PostOrderIterator<'a, 'b> {
    type Item = &'b Expression<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (index, discovered) = self.stack.pop()?;
            let expression = self.program.expressions.get(index)?;

            match expression {
                Expression::Terminal(_) => return Some(expression),
                Expression::Internal(_) if discovered => return Some(expression),
                Expression::Internal(internal) => {
                    self.stack.push((index, true));

                    for child in internal.children().iter().rev() {
                        self.stack.push((child.0, false));
                    }
                }
            }
        }
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
        let actual: Vec<Expression<'static>> = program.iter().cloned().collect();

        assert_eq!(expected, actual);
    }
}
