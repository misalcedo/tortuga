//! Re-usable grammar component for non-empty lists.

use std::fmt::Debug;

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
}
