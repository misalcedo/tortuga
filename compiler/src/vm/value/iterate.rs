use crate::Value;
use std::slice;

pub enum Iter<'a> {
    Group(slice::Iter<'a, Value>),
    Single(Option<&'a Value>),
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Group(inner) => inner.next_back(),
            Iter::Single(inner) => inner.take(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Group(inner) => inner.next(),
            Iter::Single(inner) => inner.take(),
        }
    }
}
