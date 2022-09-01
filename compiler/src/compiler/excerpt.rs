use crate::compiler::{Location, Token};
use std::collections::Bound;
use std::ops::{Index, RangeBounds};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Excerpt {
    start: Bound<Location>,
    end: Bound<Location>,
}

impl<R> From<R> for Excerpt
where
    R: RangeBounds<Location>,
{
    fn from(range: R) -> Self {
        let start = range.start_bound().cloned();
        let end = range.end_bound().cloned();

        Excerpt { start, end }
    }
}

impl Index<&Excerpt> for str {
    type Output = str;

    fn index(&self, index: &Excerpt) -> &Self::Output {
        match (index.start, index.end) {
            (Bound::Excluded(start), Bound::Excluded(end)) => {
                &self[start.offset() + 1..end.offset()]
            }
            (Bound::Excluded(start), Bound::Included(end)) => {
                &self[start.offset() + 1..=end.offset()]
            }
            (Bound::Excluded(start), Bound::Unbounded) => &self[start.offset() + 1..],
            (Bound::Included(start), Bound::Excluded(end)) => &self[start.offset()..end.offset()],
            (Bound::Included(start), Bound::Included(end)) => &self[start.offset()..=end.offset()],
            (Bound::Included(start), Bound::Unbounded) => &self[start.offset()..],
            (Bound::Unbounded, Bound::Excluded(end)) => &self[..end.offset()],
            (Bound::Unbounded, Bound::Included(end)) => &self[..=end.offset()],
            (Bound::Unbounded, Bound::Unbounded) => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excerpt() {
        let text = "Hello, World!";
        let excerpt = Excerpt::from(Location::from("Hello, ")..Location::from("Hello, World"));

        assert_eq!(&text[&excerpt], "World");
    }

    #[test]
    fn excerpt_included() {
        let text = "Hello, World!";
        let excerpt = Excerpt::from(Location::from("Hello, ")..=Location::from("Hello, World"));

        assert_eq!(&text[&excerpt], "World!");
    }

    #[test]
    fn excerpt_unbounded() {
        let text = "Hello, World!";
        let excerpt = Excerpt::from(..);

        assert_eq!(&text[&excerpt], text);
    }
}
