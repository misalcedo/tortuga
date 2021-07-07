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
    results: Vec<Result>,
}

impl FunctionType {
    pub fn parameters(&self) -> Vec<&ValueType> {
        self.parameters.iter().map(Parameter::value_type).collect()
    }

    pub fn results(&self) -> Vec<&ValueType> {
        self.results.iter().map(Result::value_type).collect()
    }
}

pub struct Local {
    id: Identifier,
    value_type: ValueType,
}

pub struct LocalIndex(Identifier);

pub struct Parameter {
    id: Identifier,
    value_type: ValueType,
}

impl Parameter {
    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }
}

pub struct Result {
    value_type: ValueType,
}

impl Result {
    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }
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
