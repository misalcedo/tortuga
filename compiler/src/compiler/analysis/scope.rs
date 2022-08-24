use crate::collections::IndexedSet;
use crate::compiler::analysis::capture::Capture;
use crate::compiler::analysis::local::Local;
use crate::compiler::analysis::value::Value;
use crate::compiler::grammar::Identifier;
use crate::{Function, Operation, ToCode};

#[derive(Clone, Debug, Default)]
pub struct Scope<'a> {
    depth: usize,
    function: usize,
    arity: usize,
    locals: IndexedSet<Identifier<'a>, Local<'a>>,
    captures: Vec<Capture>,
}

impl<'a> Scope<'a> {
    pub fn new(&self, function: usize) -> Self {
        Scope {
            function,
            depth: self.depth + 1,
            arity: Default::default(),
            locals: Default::default(),
            captures: Default::default(),
        }
    }

    pub fn push_operation(&mut self, operation: Operation) {
        self.code.push(operation);
    }

    pub fn push_local(&mut self, name: Identifier<'a>) -> usize {
        self.locals
            .insert_with(name, |index| Local::new(name, index + 1))
    }

    pub fn push_capture(&mut self, enclosing_index: usize, local: bool, kind: Value) -> usize {
        let index = self.captures.len();

        self.captures
            .push(Capture::new(enclosing_index, index, local, kind));

        index
    }

    pub fn capture_local(&mut self, local: &Local<'a>) {
        if let Some(local) = self.locals.get_mut(local.index()) {
            local.capture();
        }
    }

    pub fn set_arity(&mut self, arity: usize) {
        self.arity = arity;
    }

    pub fn resolve_local(&self, name: &Identifier<'a>) -> Option<Local<'a>> {
        self.locals.lookup(name).cloned()
    }

    pub fn local_mut(&mut self, index: usize) -> Option<&mut Local<'a>> {
        self.locals.get_mut(index)
    }

    pub fn function(&self) -> usize {
        self.function
    }

    pub fn locals(&self) -> usize {
        self.locals.len()
    }

    pub fn captures(&self) -> usize {
        self.captures.len()
    }

    pub fn capture_offsets(&self) -> impl Iterator<Item = usize> + '_ {
        self.captures.iter().map(|c| c.parent())
    }

    pub fn capture(&self, index: usize) -> Option<Capture> {
        self.captures.get(index).cloned()
    }
}

impl<'a> From<Scope<'a>> for Function {
    fn from(context: Scope<'a>) -> Self {
        let captures: Vec<Capture> = context.captures.into();
        let captures: Vec<bool> = captures.iter().map(|c| c.local()).collect();

        Function::new(
            context.arity,
            context.locals.len(),
            context.code.to_code(),
            captures,
        )
    }
}
