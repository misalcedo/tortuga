use crate::analyzer::local::Local;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum ScopeKind {
    #[default]
    Script,
    Function,
}

#[derive(Clone, Debug)]
pub struct ScopeContext<'a> {
    name: Option<&'a str>,
    depth: usize,
    kind: ScopeKind,
    locals: HashMap<Option<&'a str>, Local<'a>>,
}

impl<'a> Default for ScopeContext<'a> {
    fn default() -> Self {
        ScopeContext {
            name: None,
            kind: ScopeKind::Script,
            locals: HashMap::from([(None, Local::initialized(None, 0))]),
            depth: 0,
        }
    }
}

impl<'a> ScopeContext<'a> {
    pub fn new(name: &'a str, kind: ScopeKind) -> Self {
        ScopeContext {
            kind,
            name: Some(name),
            locals: HashMap::from([(None, Local::initialized(Some(name), 0))]),
            depth: 0,
        }
    }

    pub fn kind(&self) -> &ScopeKind {
        &self.kind
    }

    #[must_use]
    pub fn add_local(&mut self, name: &'a str) -> bool {
        self.locals.insert(Some(name), Local::from(name)).is_none()
    }
}
