use crate::web_assembly::{Identifier, TableType};

pub struct Table {
    id: Identifier,
    signature: TableType,
}

pub struct TableIndex(Identifier);

pub struct TableUse(TableIndex);
