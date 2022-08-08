use crate::{Function, Number, Text, Value};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program {
    content: Vec<u8>,
    numbers: Vec<Number>,
    texts: Vec<Text>,
    functions: Vec<Function>,
}

impl Program {
    pub fn new(
        content: Vec<u8>,
        numbers: Vec<Number>,
        texts: Vec<Text>,
        functions: Vec<Function>,
    ) -> Self {
        Program {
            content,
            numbers,
            texts,
            functions,
        }
    }

    pub fn content(&self, start: usize, size: usize) -> &[u8] {
        &self.content.as_slice()[start..start + size]
    }

    pub fn number(&self, index: usize) -> Option<&Number> {
        self.numbers.get(index)
    }

    pub fn text(&self, index: usize) -> Option<&Text> {
        self.texts.get(index)
    }

    pub fn function(&self, index: usize) -> Option<&Function> {
        self.functions.get(index)
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}
