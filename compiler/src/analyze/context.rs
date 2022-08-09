use crate::analyze::local::Local;
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
    local_indices: HashMap<Option<&'a str>, usize>,
    locals: Vec<Local<'a>>,
}

impl<'a> Default for ScopeContext<'a> {
    fn default() -> Self {
        ScopeContext {
            name: None,
            kind: ScopeKind::Script,
            local_indices: HashMap::from([(None, 0)]),
            locals: vec![Local::initialized(None, 0)],
            depth: 0,
        }
    }
}

impl<'a> ScopeContext<'a> {
    pub fn new(name: &'a str, kind: ScopeKind) -> Self {
        ScopeContext {
            kind,
            name: Some(name),
            local_indices: HashMap::from([(None, 0)]),
            locals: vec![Local::initialized(None, 0)],
            depth: 0,
        }
    }

    pub fn kind(&self) -> &ScopeKind {
        &self.kind
    }

    pub fn add_local(&mut self, name: &'a str) {
        let index = self.locals.len();

        self.locals.push(Local::from(name));
        self.local_indices.insert(Some(name), index);
    }

    pub fn resolve_local(&self, name: &str) -> Option<&Local<'a>> {
        let index = *self.local_indices.get(&Some(name))?;

        self.locals.get(index)
    }
}
