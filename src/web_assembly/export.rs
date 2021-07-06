use crate::web_assembly::{FunctionIndex, GlobalIndex, MemoryIndex, Name, TableIndex};

pub struct Export {
    name: Name,
    description: ExportDescription,
}

pub enum ExportDescription {
    Function(FunctionIndex),
    Table(TableIndex),
    Memory(MemoryIndex),
    Global(GlobalIndex),
}
