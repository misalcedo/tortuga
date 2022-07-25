//! Re-usable grammar component for non-empty lists.

use std::fmt::Debug;
use std::iter::Chain;

/// A non-empty `List` of items.
/// By default, the `Head` and `Tail` of a `List` have the same type, but they may differ.  
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct List<H, T = H>(H, Vec<T>)
where
    H: Clone + Debug + Eq + PartialEq,
    T: Clone + Debug + Eq + PartialEq;

impl<Head, Tail> List<Head, Tail>
where
    Head: Clone + Debug + Eq + PartialEq,
    Tail: Clone + Debug + Eq + PartialEq,
{
    /// Creates a new instance of a non-empty `List`.
    pub fn new<H: Into<Head>>(head: H, tail: Vec<Tail>) -> Self {
        List(head.into(), tail)
    }

    /// The head of this `List`.
    pub fn head(&self) -> &Head {
        &self.0
    }

    /// The tail (i.e. rest) of this `List`.
    pub fn tail(&self) -> &[Tail] {
        self.1.as_slice()
    }

    /// The number of total elements in this [`List`].
    pub fn len(&self) -> usize {
        1 + self.tail().len()
    }

    /// Tests whether this [`List`] has no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<Type> List<Type, Type>
where
    Type: Clone + Debug + Eq + PartialEq,
{
    /// Returns an [`Iterator`] over this [`List`].
    pub fn iter(&self) -> impl Iterator<Item = &Type> {
        Some(&self.0).into_iter().chain(self.1.as_slice())
    }
}

impl<Type> IntoIterator for List<Type, Type>
where
    Type: Clone + Debug + Eq + PartialEq,
{
    type Item = Type;
    type IntoIter = Chain<std::option::IntoIter<Self::Item>, std::vec::IntoIter<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self.0).into_iter().chain(self.1)
    }
}
