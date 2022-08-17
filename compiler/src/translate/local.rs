use super::value::Value;
use crate::grammar::Identifier;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Local<'a> {
    name: Option<Identifier<'a>>,
    offset: usize,
    depth: Option<usize>,
    is_captured: bool,
    kind: Value,
}

impl<'a> Local<'a> {
    pub fn initialized(kind: Value) -> Self {
        Local {
            name: None,
            offset: 0,
            depth: Some(0),
            is_captured: false,
            kind,
        }
    }

    pub fn new(name: Identifier<'a>, offset: usize) -> Self {
        Local {
            name: Some(name),
            offset,
            depth: None,
            kind: Value::Uninitialized(offset, None),
            is_captured: false,
        }
    }

    pub fn initialize(&mut self, depth: usize, kind: Value) {
        self.depth = Some(depth);
        self.kind = kind;
    }

    pub fn depth(&self) -> Option<usize> {
        self.depth
    }

    pub fn name(&self) -> Option<Identifier<'a>> {
        self.name
    }

    pub fn kind(&self) -> &Value {
        &self.kind
    }

    pub fn offset(&self) -> usize {
        self.offset
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
        self.name.as_ref() == Some(other)
    }
}

impl<'a, 'b> PartialOrd<Identifier<'b>> for Local<'a> {
    fn partial_cmp(&self, other: &Identifier<'b>) -> Option<Ordering> {
        self.name?.partial_cmp(other)
    }
}

impl<'a> Hash for Local<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl<'a> From<Local<'a>> for Value {
    fn from(local: Local<'a>) -> Self {
        local.kind
    }
}
