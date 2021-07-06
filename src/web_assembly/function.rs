use crate::web_assembly::{Expression, Identifier, ValueType};

pub struct Function {
    id: Identifier,
    signature: TypeUse,
    locals: Vec<Local>,
    expression: Expression,
}

pub struct FunctionIndex(Identifier);

pub struct FunctionType {
    parameters: Vec<Parameter>,
    result: Vec<Result>,
}

pub struct Local {
    id: Identifier,
    value_type: ValueType,
}

pub struct Parameter {
    id: Identifier,
    value_type: ValueType,
}

pub struct Result {
    value_type: ValueType,
}

pub struct Type {
    id: Identifier,
    function_type: FunctionType,
}

pub struct TypeIndex(Identifier);

pub struct TypeUse {
    index: TypeIndex,
    parameters: Vec<Parameter>,
    result: Vec<Result>,
}
