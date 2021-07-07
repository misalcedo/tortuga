use crate::web_assembly::{Identifier, Limit, ReferenceType};

pub struct Table {
    id: Identifier,
    signature: TableType,
}

pub struct TableIndex(Identifier);

pub struct TableUse(TableIndex);

#[derive(Copy, Clone)]
pub struct TableType {
    limits: Limit,
    reference_type: ReferenceType,
}

impl TableType {
    pub fn limits(&self) -> &Limit {
        &self.limits
    }

    pub fn reference_type(&self) -> &ReferenceType {
        &self.reference_type
    }
}
