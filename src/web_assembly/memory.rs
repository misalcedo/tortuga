use crate::web_assembly::{Identifier, Limit};

pub struct Memory {}

pub struct MemoryIndex(Identifier);

pub struct MemoryUse(MemoryIndex);

pub struct MemoryType {
    limits: Limit,
}
