use crate::grammar::{Expression, Program};

#[derive(Clone, Debug, PartialEq)]
pub struct PreOrderIterator<'a, 'b> {
    program: &'b Program<'a>,
    stack: Vec<usize>,
}

impl<'a, 'b> From<&'b Program<'a>> for PreOrderIterator<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PreOrderIterator {
            program,
            stack: program.roots.iter().map(|r| r.0).collect(),
        }
    }
}

impl<'a, 'b> Iterator for PreOrderIterator<'a, 'b> {
    type Item = &'b Expression<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;
        let expression = self.program.expressions.get(index)?;

        if let Expression::Internal(internal) = expression {
            for child in internal.children().iter().rev() {
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

impl<'a, 'b> From<&'b Program<'a>> for PostOrderIterator<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PostOrderIterator {
            program,
            stack: program.roots.iter().rev().map(|i| (i.0, false)).collect(),
        }
    }
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
