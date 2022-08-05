use crate::grammar::{Expression, InternalKind, Program};

#[derive(Clone, Debug, PartialEq)]
pub struct PreOrderIterator<'a, 'b> {
    program: &'b Program<'a>,
    stack: Vec<(usize, usize)>,
}

impl<'a, 'b> From<&'b Program<'a>> for PreOrderIterator<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PreOrderIterator {
            program,
            stack: program.roots.iter().rev().map(|&r| (0, r)).collect(),
        }
    }
}

impl<'a, 'b> Iterator for PreOrderIterator<'a, 'b> {
    type Item = (usize, &'b Expression<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let (scope_depth, index) = self.stack.pop()?;
        let expression = self.program.expressions.get(index)?;

        if let Expression::Internal(internal) = expression {
            let increment = if internal.kind() == &InternalKind::Block {
                1
            } else {
                0
            };

            for child in internal.children().iter().rev() {
                self.stack.push((scope_depth + increment, child.0));
            }
        }

        Some((scope_depth, expression))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PostOrderIterator<'a, 'b> {
    program: &'b Program<'a>,
    stack: Vec<(usize, usize, bool)>,
}

impl<'a, 'b> From<&'b Program<'a>> for PostOrderIterator<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PostOrderIterator {
            program,
            stack: program.roots.iter().rev().map(|&i| (0, i, false)).collect(),
        }
    }
}

impl<'a, 'b> Iterator for PostOrderIterator<'a, 'b> {
    type Item = (usize, &'b Expression<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (scope_depth, index, discovered) = self.stack.pop()?;
            let expression = self.program.expressions.get(index)?;

            match expression {
                Expression::Terminal(_) => return Some((scope_depth, expression)),
                Expression::Internal(_) if discovered => return Some((scope_depth, expression)),
                Expression::Internal(internal) => {
                    self.stack.push((scope_depth, index, true));

                    let increment = if internal.kind() == &InternalKind::Block {
                        1
                    } else {
                        0
                    };

                    for child in internal.children().iter().rev() {
                        self.stack.push((scope_depth + increment, child.0, false));
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IteratorWithoutScopeDepth<Iterator> {
    inner: Iterator,
}

pub trait WithoutScopeDepth: Sized {
    fn without_scope_depth(self) -> IteratorWithoutScopeDepth<Self>;
}

impl<'a, 'b, I> WithoutScopeDepth for I
where
    'a: 'b,
    I: Iterator<Item = (usize, &'b Expression<'a>)>,
{
    fn without_scope_depth(self) -> IteratorWithoutScopeDepth<Self> {
        IteratorWithoutScopeDepth { inner: self }
    }
}

impl<'a, 'b, I> Iterator for IteratorWithoutScopeDepth<I>
where
    'a: 'b,
    I: Iterator<Item = (usize, &'b Expression<'a>)>,
{
    type Item = &'b Expression<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.1)
    }
}
