use crate::translate::value::Value;
use tortuga_executable::Function;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypedFunction {
    function: Function,
    parameters: Value,
    results: Value,
}

impl TypedFunction {
    pub fn new(function: Function, parameters: Value, results: Value) -> Self {
        TypedFunction {
            function,
            parameters,
            results,
        }
    }
    pub fn kind(&self) -> Value {
        Value::function(self.parameters.clone(), self.results.clone())
    }

    pub fn parameters(&self) -> &Value {
        &self.parameters
    }

    pub fn results(&self) -> &Value {
        &self.results
    }
}

impl From<TypedFunction> for Function {
    fn from(function: TypedFunction) -> Self {
        function.function
    }
}
