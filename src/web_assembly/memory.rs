use crate::web_assembly::{Identifier, Limit};

pub struct Memory {
    id: Identifier,
    signature: MemoryType,
}

pub struct MemoryIndex(Identifier);

pub struct MemoryUse(MemoryIndex);

#[derive(Copy, Clone)]
pub struct MemoryType {
    limits: Limit,
}

impl MemoryType {
    pub fn limits(&self) -> &Limit {
        &self.limits
    }
}
