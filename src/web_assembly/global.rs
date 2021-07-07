use crate::web_assembly::{Expression, Identifier, ValueType};

pub struct Global {
    id: Identifier,
    signature: GlobalType,
    expression: Expression,
}

pub struct GlobalIndex(Identifier);

pub enum GlobalType {
    Constant(ValueType),
    Variable(ValueType),
}
