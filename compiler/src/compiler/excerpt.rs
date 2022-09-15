use crate::compiler::Location;
use std::collections::Bound;
use std::fmt::{Display, Formatter};
use std::ops::{Index, RangeBounds};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Excerpt {
    start: Bound<Location>,
    end: Bound<Location>,
}

impl Default for Excerpt {
    fn default() -> Self {
        Excerpt {
            start: Bound::Included(Location::default()),
            end: Bound::Excluded(Location::default()),
        }
    }
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

impl From<&str> for Excerpt {
    fn from(start: &str) -> Self {
        let start = Location::from(start);

        Excerpt {
            start: Bound::Included(start),
            end: Bound::Unbounded,
        }
    }
}

impl Excerpt {
    pub fn find(source: &str, excerpt: &str) -> Self {
        match source.find(excerpt) {
            None => Self::default(),
            Some(start) => {
                let start = Location::from(&source[..start]);
                let end = start + excerpt;

                Excerpt::from(start..end)
            }
        }
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

impl Display for Excerpt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.start {
            Bound::Included(start) => write!(f, "[{}", start)?,
            Bound::Excluded(start) => write!(f, "({}", start)?,
            Bound::Unbounded => {}
        };

        write!(f, ", ")?;

        match self.start {
            Bound::Included(end) => write!(f, "{}]", end),
            Bound::Excluded(end) => write!(f, "{})", end),
            Bound::Unbounded => Ok(()),
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

    #[test]
    fn excerpt_default() {
        let text = "Hello, World!";
        let excerpt = Excerpt::default();

        assert_eq!(&text[&excerpt], "");
    }

    #[test]
    fn excerpt_find() {
        let text = "Hello, World!";
        let excerpt = Excerpt::find(text, "World");

        assert_eq!(&text[&excerpt], "World");
    }
}
