use crate::web_assembly::{Identifier, ValueType};

pub struct Global {}

pub struct GlobalIndex(Identifier);

pub enum GlobalType {
    Constant(ValueType),
    Variable(ValueType),
}
