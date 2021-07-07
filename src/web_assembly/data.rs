use crate::web_assembly;
use crate::web_assembly::Identifier;
use crate::web_assembly::MemoryUse;
use crate::web_assembly::Offset;

/// Data segments allow for an optional memory index to identify the memory to initialize.
/// The data is written as a string, which may be split up into a possibly empty sequence of
/// individual string literals.
pub struct Data {
    id: Identifier,
    memory: MemoryUse,
    offset: Offset,
    string: DataString,
}

pub struct DataString {
    strings: Vec<web_assembly::String>,
}

pub struct DataIndex(Identifier);
