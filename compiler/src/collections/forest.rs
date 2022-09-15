use crate::collections::Tree;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Forest<Data, TreeData> {
    data: Data,
    trees: Vec<Tree<TreeData>>,
}

impl<D, TD> From<D> for Forest<D, TD> {
    fn from(data: D) -> Self {
        Forest {
            data,
            trees: vec![],
        }
    }
}

impl<D, TD> Forest<D, TD> {
    pub fn insert<T>(&mut self, tree: T) -> &mut Tree<TD>
    where
        T: Into<Tree<TD>>,
    {
        let index = self.trees.len();

        self.trees.push(tree.into());

        &mut self.trees[index]
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.trees.len()
    }

    pub fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }

    pub fn trees(&self) -> RootsIterator<'_, TD> {
        self.trees.as_slice().into()
    }

    pub fn iter(&self) -> Iter<'_, TD> {
        Iter::from(self.trees.as_slice())
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

#[derive(Clone, Debug)]
pub struct RootsIterator<'a, D>(std::slice::Iter<'a, Tree<D>>);

impl<'a, D> From<&'a [Tree<D>]> for RootsIterator<'a, D> {
    fn from(tree: &'a [Tree<D>]) -> Self {
        RootsIterator(tree.iter())
    }
}

impl<'a, D> Iterator for RootsIterator<'a, D>
where
    D: 'a,
{
    type Item = Node<'a, D>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Node::from(self.0.next()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math() {
        let source = "x = 3 + 2\nx * 5";
        let mut program = Forest::from(source);

        let equality = program.insert("=");
        equality.insert("x");

        let sum = equality.insert("+");
        sum.insert("3");
        sum.insert("2");

        let product = program.insert("*");
        product.insert("x");
        product.insert("5");

        assert_eq!(program.len(), 2);
        assert!(!program.is_empty());

        let expressions: Vec<&str> = program.iter().pre_order().map(|n| *n.data).collect();

        assert_eq!(expressions, vec!["=", "x", "+", "3", "2", "*", "x", "5"])
    }
}
