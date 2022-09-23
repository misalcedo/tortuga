use crate::collections::tree::{Iter, Node};
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
    use crate::collections::tree::NodeIterator;

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

        let expressions: Vec<&str> = program.iter().pre_order().map(|n| *n.data()).collect();

        assert_eq!(expressions, vec!["=", "x", "+", "3", "2", "*", "x", "5"])
    }
}
