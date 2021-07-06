use crate::web_assembly::Identifier;

pub struct Function {
    id: Identifier,
}

pub struct FunctionIndex(Identifier);

pub struct Type {}

pub struct TypeIndex(Identifier);

pub struct TypeUse(TypeIndex);
