use crate::collections::IndexedSet;
use crate::compiler::analysis::{Capture, Local, Type};
use std::borrow::Cow;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Function<'a> {
    depth: usize,
    index: usize,
    parameters: Type,
    results: Type,
    captures: IndexedSet<Cow<'a, str>, Capture>,
    locals: IndexedSet<Cow<'a, str>, Local<'a>>,
}
