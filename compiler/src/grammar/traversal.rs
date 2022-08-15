use crate::grammar::{Expression, Program};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node<'a, 'b> {
    discovered: bool,
    program: &'b Program<'a>,
    expression: &'b Expression<'a>,
}

impl<'a, 'b> Node<'a, 'b>
where
    'a: 'b,
{
    fn new(program: &'b Program<'a>, index: usize) -> Option<Self> {
        let expression = program.expressions.get(index)?;

        Some(Node {
            discovered: false,
            program,
            expression,
        })
    }

    fn new_child(&self, index: usize) -> Option<Self> {
        let expression = self.program.expressions.get(index)?;

        Some(Node {
            discovered: false,
            program: self.program,
            expression,
        })
    }

    pub fn discovered(&self) -> bool {
        self.discovered
    }

    pub fn program(&self) -> &'b Program<'a> {
        self.program
    }

    pub fn expression(&self) -> &'b Expression<'a> {
        self.expression
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Iter<'a, 'b> {
    stack: Vec<Node<'a, 'b>>,
}

impl<'a, 'b> From<&'b Program<'a>> for Iter<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        Iter {
            stack: program
                .roots
                .iter()
                .rev()
                .filter_map(|&i| Node::new(program, i))
                .collect(),
        }
    }
}

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = Node<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        if node.discovered {
            Some(node)
        } else {
            self.stack.push(Node {
                discovered: true,
                ..node
            });

            for reference in node.expression.children().iter().rev() {
                if let Some(child) = node.new_child(reference.0) {
                    self.stack.push(child);
                }
            }

            Some(node)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreOrderIterator<'a, 'b>(Iter<'a, 'b>);

impl<'a, 'b> From<&'b Program<'a>> for PreOrderIterator<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PreOrderIterator(program.into())
    }
}

impl<'a, 'b> Iterator for PreOrderIterator<'a, 'b> {
    type Item = Node<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.0.next()?;

        while node.discovered {
            node = self.0.next()?;
        }

        Some(node)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PostOrderIterator<'a, 'b>(Iter<'a, 'b>);

impl<'a, 'b> From<&'b Program<'a>> for PostOrderIterator<'a, 'b> {
    fn from(program: &'b Program<'a>) -> Self {
        PostOrderIterator(program.into())
    }
}

impl<'a, 'b> Iterator for PostOrderIterator<'a, 'b> {
    type Item = Node<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.0.next()?;

        while !node.discovered {
            node = self.0.next()?;
        }

        Some(node)
    }
}
