use crate::web_assembly::{ConstantExpression, Identifier, ValueType};

pub struct Global {
    id: Identifier,
    signature: GlobalType,
    expression: ConstantExpression,
}

pub struct GlobalIndex(Identifier);

pub enum GlobalType {
    Constant(ValueType),
    Variable(ValueType),
}
