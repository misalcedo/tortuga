use crate::web_assembly::{Identifier, MemoryType};

pub struct Memory {
    id: Identifier,
    signature: MemoryType,
}

pub struct MemoryIndex(Identifier);

pub struct MemoryUse(MemoryIndex);
