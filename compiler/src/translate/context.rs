use crate::grammar::Identifier;
use crate::translate::indices::IndexedSet;
use crate::translate::local::Local;
use crate::translate::value::Value;

#[derive(Clone, Debug)]
pub struct ScopeContext<'a> {
    name: Option<&'a str>,
    depth: usize,
    locals: IndexedSet<Option<Identifier<'a>>, Local<'a>>,
}

impl<'a> Default for ScopeContext<'a> {
    fn default() -> Self {
        ScopeContext {
            name: None,
            locals: IndexedSet::from([(None, Local::initialized(Value::Closure))]),
            depth: 0,
        }
    }
}

impl<'a> ScopeContext<'a> {
    pub fn new(name: &'a str) -> Self {
        ScopeContext {
            name: Some(name),
            locals: IndexedSet::from([(None, Local::initialized(Value::Closure))]),
            depth: 0,
        }
    }

    pub fn add_local(&mut self, name: Identifier<'a>) -> usize {
        self.locals
            .insert_with(Some(name), |offset| Local::new(name, offset))
    }

    pub fn resolve_local(&self, name: &Identifier<'a>) -> Option<&Local<'a>> {
        self.locals.lookup(&Some(*name))
    }
}
