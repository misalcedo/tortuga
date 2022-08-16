use crate::translate::value::Value;
use tortuga_executable::Function;

pub struct TypedFunction {
    function: Function,
    parameters: Vec<Value>,
    results: Vec<Value>,
}

impl TypedFunction {
    pub fn new(function: Function, parameters: Vec<Value>, results: Vec<Value>) -> Self {
        TypedFunction {
            function,
            parameters,
            results,
        }
    }
    pub fn kind(&self) -> Value {
        Value::Function(self.parameters.clone(), self.results.clone())
    }

    pub fn parameters(&self) -> &[Value] {
        self.parameters.as_slice()
    }

    pub fn results(&self) -> &[Value] {
        self.results.as_slice()
    }
}

impl From<TypedFunction> for Function {
    fn from(function: TypedFunction) -> Self {
        function.function
    }
}
