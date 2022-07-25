use crate::Value;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program {
    content: Vec<u8>,
    constants: Vec<Value>,
}

impl Program {
    pub fn new(content: Vec<u8>, constants: Vec<Value>) -> Self {
        Program { content, constants }
    }

    pub fn content(&self, start: usize, size: usize) -> &[u8] {
        &self.content.as_slice()[start..start + size]
    }

    pub fn constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}
