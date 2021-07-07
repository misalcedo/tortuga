use crate::web_assembly::{Expression, Identifier, ValueType};

pub struct Global {
    id: Identifier,
    signature: GlobalType,
    expression: Expression,
}

pub struct GlobalIndex(Identifier);

#[derive(Copy, Clone)]
pub struct GlobalType {
    value_type: ValueType,
    mutability: Mutability,
}

impl GlobalType {
    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }

    pub fn mutability(&self) -> &Mutability {
        &self.mutability
    }
}

#[derive(Copy, Clone)]
pub enum Mutability {
    Constant,
    Variable,
}
