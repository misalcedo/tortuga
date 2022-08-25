use crate::collections::IndexedSet;
use crate::compiler::analysis::{Capture, Local, Type};
use std::borrow::Cow;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Function<'a> {
    depth: usize,
    index: usize,
    parameters: Type,
    results: Type,
    captures: IndexedSet<Cow<'a, str>, Capture<'a>>,
    locals: IndexedSet<Cow<'a, str>, Local<'a>>,
}

impl<'a> Function<'a> {
    pub fn push_local(&mut self, name: Cow<'a, str>) -> usize {
        self.locals
            .insert_with(name.clone(), |index| Local::new(name, index + 1))
    }

    pub fn capture_local(&mut self, local: &mut Local<'a>) -> usize {
        self.captures.insert_with(local.name().clone(), |offset| {
            Capture::new_local(local, offset)
        })
    }

    pub fn capture_transitive(&mut self, capture: &Capture<'a>) -> usize {
        self.captures.insert_with(capture.name().clone(), |offset| {
            Capture::new_transitive(capture, offset)
        })
    }

    pub fn resolve_local(&self, name: &str) -> Option<&Local<'a>> {
        self.locals.lookup(name)
    }

    pub fn resolve_local_mut(&mut self, name: &str) -> Option<&mut Local<'a>> {
        self.locals.lookup_mut(name)
    }

    pub fn resolve_capture(&self, name: &str) -> Option<&Capture<'a>> {
        self.captures.lookup(name)
    }

    pub fn resolve_capture_mut(&mut self, name: &str) -> Option<&mut Capture<'a>> {
        self.captures.lookup_mut(name)
    }
}
