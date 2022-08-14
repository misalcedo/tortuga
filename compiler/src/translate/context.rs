use crate::grammar::Identifier;
use crate::translate::capture::Capture;
use crate::translate::indices::IndexedSet;
use crate::translate::local::Local;
use tortuga_executable::{Code, Operation};

#[derive(Clone, Debug)]
pub struct ScopeContext<'a> {
    name: Option<&'a str>,
    depth: usize,
    code: Vec<u8>,
    parameters: IndexedSet<Identifier<'a>, Local<'a>>,
    locals: IndexedSet<Identifier<'a>, Local<'a>>,
    captures: IndexedSet<Option<Identifier<'a>>, Capture>,
}

impl<'a> Default for ScopeContext<'a> {
    fn default() -> Self {
        ScopeContext {
            name: None,
            depth: 0,
            code: vec![],
            parameters: Default::default(),
            locals: IndexedSet::default(),
            captures: IndexedSet::default(),
        }
    }
}

impl<'a> ScopeContext<'a> {
    pub fn new(name: &'a str) -> Self {
        ScopeContext {
            name: Some(name),
            depth: 0,
            code: vec![],
            parameters: Default::default(),
            locals: IndexedSet::default(),
            captures: Default::default(),
        }
    }

    pub fn add_operation(&mut self, operation: Operation) {
        self.code.push_operation(&operation);
    }

    pub fn add_local(&mut self, name: Identifier<'a>) -> usize {
        self.locals
            .insert_with(name, |index| Local::new(name, index + 1))
    }

    pub fn resolve_local(&self, name: &Identifier<'a>) -> Option<&Local<'a>> {
        self.locals.lookup(name)
    }

    pub fn local_mut(&mut self, index: usize) -> Option<&mut Local<'a>> {
        self.locals.get_mut(index)
    }

    pub fn locals(&self) -> usize {
        self.locals.len()
    }
}
