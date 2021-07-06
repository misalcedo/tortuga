use crate::web_assembly::{Identifier, Limit, ReferenceType};

pub struct Table {}

pub struct TableIndex(Identifier);

pub struct TableUse(TableIndex);

pub struct TableType {
    limits: Limit,
    reference_type: ReferenceType,
}