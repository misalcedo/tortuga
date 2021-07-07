use crate::web_assembly::{Expression, FunctionType, Identifier, ResultType, ValueType};

pub struct Function {
    id: Identifier,
    signature: TypeUse,
    locals: Vec<Local>,
    expression: Expression,
}

pub struct FunctionIndex(Identifier);

pub struct Local {
    id: Identifier,
    value_type: ValueType,
}

pub struct LocalIndex(Identifier);

pub struct Type {
    id: Identifier,
    function_type: FunctionType,
}

pub struct TypeIndex(Identifier);

pub struct TypeUse {
    index: TypeIndex,
    parameters: ResultType,
    result: ResultType,
}
