use super::types::Type;
use crate::compiler::grammar::Identifier;
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Local<'a> {
    name: Cow<'a, str>,
    offset: usize,
    depth: Option<usize>,
    is_captured: bool,
    kind: Type,
}

impl<'a> Local<'a> {
    pub fn new(name: Identifier<'a>, offset: usize) -> Self {
        Local {
            name: name.as_str().into(),
            offset,
            depth: None,
            kind: Type::default(),
            is_captured: false,
        }
    }

    pub fn initialize(&mut self, depth: usize, kind: Type) -> usize {
        self.depth = Some(depth);
        self.kind = kind;
        self.offset
    }

    pub fn depth(&self) -> Option<usize> {
        self.depth
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &Type {
        &self.kind
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn index(&self) -> usize {
        self.offset.checked_sub(1).unwrap_or(0)
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

impl<'a, 'b> PartialEq<Identifier<'b>> for Local<'a> {
    fn eq(&self, other: &Identifier<'b>) -> bool {
        self.name.borrow() == other.as_str()
    }
}

impl<'a, 'b> PartialOrd<Identifier<'b>> for Local<'a> {
    fn partial_cmp(&self, other: &Identifier<'b>) -> Option<Ordering> {
        self.name.borrow().partial_cmp(other.as_str())
    }
}

impl<'a> Hash for Local<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
