use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Local<'a> {
    name: Option<&'a str>,
    depth: Option<usize>,
    is_captured: bool,
}

impl<'a> From<&'a str> for Local<'a> {
    fn from(name: &'a str) -> Self {
        Local {
            name: Some(name),
            depth: None,
            is_captured: false,
        }
    }
}
impl<'a> Local<'a> {
    pub fn initialized(name: Option<&'a str>, depth: usize) -> Self {
        Local {
            name,
            depth: Some(depth),
            is_captured: false,
        }
    }

    pub fn depth(&self) -> Option<usize> {
        self.depth
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn is_captured(&self) -> bool {
        self.is_captured
    }

    pub fn mark_initialized(&mut self, depth: usize) {
        self.depth = Some(depth);
    }

    pub fn capture(&mut self) {
        self.is_captured = true;
    }
}

impl<'a> PartialEq<str> for Local<'a> {
    fn eq(&self, other: &str) -> bool {
        self.name == Some(other)
    }
}

impl<'a> PartialOrd<str> for Local<'a> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.name?.partial_cmp(other)
    }
}

impl<'a> Hash for Local<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
