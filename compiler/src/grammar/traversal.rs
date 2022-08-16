use crate::grammar::{Expression, ExpressionReference, Program};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node<'a, 'b> {
    discovered: bool,
    depth: usize,
    expression: &'b Expression<'a>,
    program: &'b Program<'a>,
}

impl<'a, 'b> Node<'a, 'b>
where
    'a: 'b,
{
    pub(crate) fn new(program: &'b Program<'a>, index: usize) -> Option<Self> {
        let expression = program.expressions.get(index)?;

        Some(Node {
            discovered: false,
            depth: 0,
            program,
            expression,
        })
    }

    fn new_child(&self, index: usize) -> Option<Self> {
        let expression = self.program.expressions.get(index)?;

        Some(Node {
            discovered: false,
            program: self.program,
            depth: self.depth + 1,
            expression,
        })
    }

    pub fn children(&self) -> usize {
        self.expression.children().len()
    }

    pub fn discovered(&self) -> bool {
        self.discovered
    }

    pub fn root(&self) -> bool {
        self.depth == 0
    }

    pub fn depth(&self) -> usize {
        self.depth
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

pub trait NodeIterator<'a, 'b>: Iterator<Item = Node<'a, 'b>> + Sized
where
    'a: 'b,
{
    fn pre_order(self) -> PreOrderIterator<Self>;

    fn post_order(self) -> PostOrderIterator<Self>;
}

impl<'a, 'b, I> NodeIterator<'a, 'b> for I
where
    'a: 'b,
    I: Iterator<Item = Node<'a, 'b>>,
{
    fn pre_order(self) -> PreOrderIterator<Self> {
        PreOrderIterator(self)
    }

    fn post_order(self) -> PostOrderIterator<Self> {
        PostOrderIterator(self)
    }
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
pub struct PreOrderIterator<Iterator>(Iterator);

impl<'a, 'b> From<&'b Program<'a>> for PreOrderIterator<Iter<'a, 'b>> {
    fn from(program: &'b Program<'a>) -> Self {
        PreOrderIterator(program.into())
    }
}

impl<'a, 'b, I> Iterator for PreOrderIterator<I>
where
    'a: 'b,
    I: Iterator<Item = Node<'a, 'b>>,
{
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
pub struct PostOrderIterator<Iterator>(Iterator);

impl<'a, 'b> From<&'b Program<'a>> for PostOrderIterator<Iter<'a, 'b>> {
    fn from(program: &'b Program<'a>) -> Self {
        PostOrderIterator(program.into())
    }
}

impl<'a, 'b, I> Iterator for PostOrderIterator<I>
where
    'a: 'b,
    I: Iterator<Item = Node<'a, 'b>>,
{
    type Item = Node<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.0.next()?;

        while !node.discovered {
            node = self.0.next()?;
        }

        Some(node)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RootIterator<'a, 'b, Iterator>(&'b Program<'a>, Iterator);

impl<'a, 'b> From<&'b Program<'a>> for RootIterator<'a, 'b, std::slice::Iter<'b, usize>> {
    fn from(program: &'b Program<'a>) -> Self {
        RootIterator(program, program.roots.iter())
    }
}

impl<'a, 'b, I> Iterator for RootIterator<'a, 'b, I>
where
    'a: 'b,
    I: Iterator<Item = &'b usize>,
{
    type Item = Node<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.1.next()?;

        Node::new(self.0, *index)
    }
}
