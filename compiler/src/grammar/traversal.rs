use crate::grammar::{Expression, Program};

#[derive(Clone, Debug, PartialEq)]
pub struct PreOrderIteratorWithHeight<'a, 'b> {
    program: &'b Program<'a>,
    stack: Vec<(usize, usize)>,
}

impl<'a, 'b> From<&'b Program<'a>> for PreOrderIteratorWithHeight<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PreOrderIteratorWithHeight {
            program,
            stack: program.roots.iter().rev().map(|r| (0, r.0)).collect(),
        }
    }
}

impl<'a, 'b> Iterator for PreOrderIteratorWithHeight<'a, 'b> {
    type Item = (usize, &'b Expression<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let (height, index) = self.stack.pop()?;
        let expression = self.program.expressions.get(index)?;

        if let Expression::Internal(internal) = expression {
            for child in internal.children().iter().rev() {
                self.stack.push((height + 1, child.0));
            }
        }

        Some((height, expression))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PostOrderIteratorWithHeight<'a, 'b> {
    program: &'b Program<'a>,
    stack: Vec<(usize, usize, bool)>,
}

impl<'a, 'b> From<&'b Program<'a>> for PostOrderIteratorWithHeight<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PostOrderIteratorWithHeight {
            program,
            stack: program
                .roots
                .iter()
                .rev()
                .map(|i| (0, i.0, false))
                .collect(),
        }
    }
}

impl<'a, 'b> Iterator for PostOrderIteratorWithHeight<'a, 'b> {
    type Item = (usize, &'b Expression<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (height, index, discovered) = self.stack.pop()?;
            let expression = self.program.expressions.get(index)?;

            match expression {
                Expression::Terminal(_) => return Some((height, expression)),
                Expression::Internal(_) if discovered => return Some((height, expression)),
                Expression::Internal(internal) => {
                    self.stack.push((height, index, true));

                    for child in internal.children().iter().rev() {
                        self.stack.push((height + 1, child.0, false));
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreOrderIterator<'a, 'b> {
    inner: PreOrderIteratorWithHeight<'a, 'b>,
}

impl<'a, 'b> From<&'b Program<'a>> for PreOrderIterator<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PreOrderIterator {
            inner: PreOrderIteratorWithHeight::from(program),
        }
    }
}

impl<'a, 'b> Iterator for PreOrderIterator<'a, 'b> {
    type Item = &'b Expression<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.1)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PostOrderIterator<'a, 'b> {
    inner: PostOrderIteratorWithHeight<'a, 'b>,
}

impl<'a, 'b> From<&'b Program<'a>> for PostOrderIterator<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PostOrderIterator {
            inner: PostOrderIteratorWithHeight::from(program),
        }
    }
}

impl<'a, 'b> Iterator for PostOrderIterator<'a, 'b> {
    type Item = &'b Expression<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.1)
    }
}
