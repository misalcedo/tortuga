//! Re-usable grammar component for non-empty lists.

use std::fmt::Debug;

/// A non-empty `List` of items.
/// By default, the `Head` and `Tail` of a `List` have the same type, but they may differ.  
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct List<H, T = H>(H, Vec<T>)
where
    H: Clone + Debug + Eq + PartialEq + Ord + PartialOrd,
    T: Clone + Debug + Eq + PartialEq + Ord + PartialOrd;

impl<Head, Tail> List<Head, Tail>
where
    Head: Clone + Debug + Eq + PartialEq + Ord + PartialOrd,
    Tail: Clone + Debug + Eq + PartialEq + Ord + PartialOrd,
{
    /// Creates a new instance of a non-empty `List`.
    pub fn new(head: Head, tail: Vec<Tail>) -> Self {
        List(head, tail)
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
