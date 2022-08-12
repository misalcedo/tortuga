use crate::translate::indices::IndexedSet;
use crate::translate::local::Local;

#[derive(Clone, Debug)]
pub struct ScopeContext<'a> {
    name: Option<&'a str>,
    depth: usize,
    locals: IndexedSet<Option<&'a str>, Local<'a>>,
}

impl<'a> Default for ScopeContext<'a> {
    fn default() -> Self {
        ScopeContext {
            name: None,
            locals: IndexedSet::from([(None, Local::initialized(None, 0))]),
            depth: 0,
        }
    }
}

impl<'a> ScopeContext<'a> {
    pub fn new(name: &'a str) -> Self {
        ScopeContext {
            name: Some(name),
            locals: IndexedSet::from([(None, Local::initialized(None, 0))]),
            depth: 0,
        }
    }

    pub fn add_local(&mut self, name: &'a str) {
        self.locals.insert(Some(name), Local::from(name));
    }

    pub fn resolve_local(&self, name: &'a str) -> Option<&Local<'a>> {
        self.locals.lookup(&Some(name))
    }
}
