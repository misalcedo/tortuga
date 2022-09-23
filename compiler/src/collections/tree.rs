#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tree<Data> {
    data: Data,
    children: Vec<Self>,
}

impl<D> From<D> for Tree<D> {
    fn from(data: D) -> Self {
        Tree {
            data,
            children: vec![],
        }
    }
}

impl<D> Tree<D> {
    pub fn new(data: D, children: Vec<Self>) -> Self {
        Tree { data, children }
    }

    pub fn insert<T>(&mut self, child: T) -> &mut Tree<D>
    where
        T: Into<Self>,
    {
        let index = self.children.len();

        self.children.push(child.into());

        &mut self.children[index]
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }

    pub fn height(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(Tree::height)
            .max()
            .unwrap_or_default()
    }

    pub fn children(&self) -> &[Tree<D>] {
        self.children.as_slice()
    }

    pub fn iter(&self) -> Iter<'_, D> {
        Iter::from(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node<'a, Data> {
    discovered: bool,
    height: usize,
    data: &'a Data,
    children: &'a [Tree<Data>],
}

impl<'a, D> From<&'a Tree<D>> for Node<'a, D> {
    fn from(tree: &'a Tree<D>) -> Self {
        Node::new(tree, 0)
    }
}

impl<'a, D> Node<'a, D> {
    pub fn new(tree: &'a Tree<D>, height: usize) -> Self {
        Node {
            height,
            discovered: false,
            data: tree.data(),
            children: tree.children(),
        }
    }

    pub fn discovered(&self) -> bool {
        self.discovered
    }

    pub fn root(&self) -> bool {
        self.height == 0
    }

    pub fn leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> &'a D {
        self.data
    }

    pub fn children(
        &self,
    ) -> impl ExactSizeIterator<Item = Node<'a, D>> + DoubleEndedIterator<Item = Node<'a, D>> + '_
    {
        self.children.iter().map(|c| Node::new(c, self.height + 1))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Iter<'a, Data> {
    stack: Vec<Node<'a, Data>>,
}

pub trait NodeIterator<'a, Data: 'a>: Iterator<Item = Node<'a, Data>> + Sized {
    fn pre_order(self) -> PreOrderIterator<Self>;

    fn post_order(self) -> PostOrderIterator<Self>;
}

impl<'a, D, I> NodeIterator<'a, D> for I
where
    D: 'a,
    I: Iterator<Item = Node<'a, D>>,
{
    fn pre_order(self) -> PreOrderIterator<Self> {
        PreOrderIterator(self)
    }

    fn post_order(self) -> PostOrderIterator<Self> {
        PostOrderIterator(self)
    }
}

impl<'a, D> From<&'a Tree<D>> for Iter<'a, D> {
    fn from(tree: &'a Tree<D>) -> Self {
        Iter {
            stack: vec![Node::from(tree)],
        }
    }
}

impl<'a, D> From<&'a [Tree<D>]> for Iter<'a, D> {
    fn from(trees: &'a [Tree<D>]) -> Self {
        Iter {
            stack: trees.iter().rev().map(Node::from).collect(),
        }
    }
}

impl<'a, D> Iterator for Iter<'a, D> {
    type Item = Node<'a, D>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        if node.discovered {
            Some(node)
        } else {
            self.stack.push(Node {
                discovered: true,
                ..node
            });

            self.stack.extend(node.children().rev());

            Some(node)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreOrderIterator<Iterator>(Iterator);

impl<'a, D> From<&'a Tree<D>> for PreOrderIterator<Iter<'a, D>> {
    fn from(tree: &'a Tree<D>) -> Self {
        PreOrderIterator(tree.into())
    }
}

impl<'a, D> From<&'a [Tree<D>]> for PreOrderIterator<Iter<'a, D>> {
    fn from(tree: &'a [Tree<D>]) -> Self {
        PreOrderIterator(tree.into())
    }
}

impl<'a, D, I> Iterator for PreOrderIterator<I>
where
    D: 'a,
    I: Iterator<Item = Node<'a, D>>,
{
    type Item = Node<'a, D>;

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

impl<'a, D> From<&'a Tree<D>> for PostOrderIterator<Iter<'a, D>> {
    fn from(tree: &'a Tree<D>) -> Self {
        PostOrderIterator(tree.into())
    }
}

impl<'a, D> From<&'a [Tree<D>]> for PostOrderIterator<Iter<'a, D>> {
    fn from(tree: &'a [Tree<D>]) -> Self {
        PostOrderIterator(tree.into())
    }
}

impl<'a, D, I> Iterator for PostOrderIterator<I>
where
    D: 'a,
    I: Iterator<Item = Node<'a, D>>,
{
    type Item = Node<'a, D>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.0.next()?;

        while !node.discovered {
            node = self.0.next()?;
        }

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math() {
        let source = "x = 3 + 2\nx * 5";
        let mut program = Tree::from(source);

        let equality = program.insert("=");
        equality.insert("x");

        let sum = equality.insert("+");
        sum.insert("3");
        sum.insert("2");

        let product = program.insert("*");
        product.insert("x");
        product.insert("5");

        assert_eq!(program.height(), 4);
    }
}
