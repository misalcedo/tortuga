use crate::compiler::translate::error::ErrorKind;
use crate::compiler::translate::scope::Scope;
use crate::compiler::translate::value::Value;
use crate::compiler::TranslationError;
use crate::Function;

#[derive(Clone, Debug)]
pub struct TypedFunction<'a> {
    scope: Option<Scope<'a>>,
    parameters: Value,
    results: Value,
}

impl Default for TypedFunction<'_> {
    fn default() -> Self {
        TypedFunction {
            scope: None,
            parameters: Value::None,
            results: Value::None,
        }
    }
}

impl<'a> TypedFunction<'a> {
    pub fn new(parameters: Value, results: Value) -> Self {
        TypedFunction {
            scope: None,
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

    pub fn initialize(&mut self, scope: Scope<'a>) -> Option<Scope<'a>> {
        self.scope.replace(scope)
    }
}

impl<'a> TryFrom<TypedFunction<'a>> for Function {
    type Error = TranslationError;

    fn try_from(function: TypedFunction) -> Result<Self, Self::Error> {
        let scope = function
            .scope
            .ok_or_else(|| TranslationError::from(ErrorKind::PartiallyDeclaredFunction))?;

        Ok(scope.into())
    }
}
