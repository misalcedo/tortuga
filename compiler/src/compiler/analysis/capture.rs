use super::Type;
use crate::compiler::analysis::Local;
use std::borrow::Cow;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Capture<'a> {
    name: Cow<'a, str>,
    parent: usize,
    offset: usize,
    local: bool,
    kind: Type,
}

impl<'a> Capture<'a> {
    pub fn new_local(local: &mut Local<'a>, offset: usize) -> Self {
        local.capture();

        Capture {
            offset,
            name: local.name().clone(),
            parent: local.offset(),
            local: true,
            kind: local.kind().clone(),
        }
    }

    pub fn new_transitive(capture: &Self, offset: usize) -> Self {
        Capture {
            offset,
            name: capture.name.clone(),
            parent: capture.offset,
            local: false,
            kind: capture.kind.clone(),
        }
    }

    pub fn name(&self) -> &Cow<'a, str> {
        &self.name
    }

    pub fn parent(&self) -> usize {
        self.parent
    }

    pub fn index(&self) -> usize {
        self.offset
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn local(&self) -> bool {
        self.local
    }

    pub fn kind(&self) -> Type {
        self.kind.clone()
    }
}
