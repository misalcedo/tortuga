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
