use crate::web_assembly;
use crate::web_assembly::expression::Expression;
use crate::web_assembly::identifier::Identifier;
use crate::web_assembly::memory::MemoryUse;

/// Data segments allow for an optional memory index to identify the memory to initialize.
/// The data is written as a string, which may be split up into a possibly empty sequence of
/// individual string literals.
pub struct Data {
    id: Identifier,
    memory: MemoryUse,
    offset: Offset,
    string: DataString,
}

pub struct Offset {
    expression: Expression,
}

pub struct DataString {
    strings: Vec<web_assembly::String>,
}
