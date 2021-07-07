use crate::web_assembly::{Expression, GlobalType, Identifier};

pub struct Global {
    id: Identifier,
    signature: GlobalType,
    expression: Expression,
}

pub struct GlobalIndex(Identifier);
