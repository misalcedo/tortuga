use crate::web_assembly::{GlobalType, Identifier, MemoryType, Name, TableType, TypeUse};

pub struct Import {
    module: Name,
    name: Name,
    description: ImportDescription,
}

pub enum ImportDescription {
    Function {
        id: Identifier,
        signature: TypeUse,
    },
    Table {
        id: Identifier,
        signature: TableType,
    },
    Memory {
        id: Identifier,
        signature: MemoryType,
    },
    Global {
        id: Identifier,
        signature: GlobalType,
    },
}
