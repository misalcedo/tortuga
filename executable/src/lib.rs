extern crate core;

mod error;
mod function;
mod number;
mod operation;
mod text;

pub use error::ParseNumberError;
pub use function::Function;
pub use number::Number;
pub use operation::{Code, Operation, OperationCode, ToCode};
pub use text::Text;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Executable {
    functions: Vec<Function>,
    numbers: Vec<Number>,
    texts: Vec<Text>,
}

impl Executable {
    pub fn new<F, N, T>(functions: F, numbers: N, texts: T) -> Self
    where
        F: Into<Vec<Function>>,
        N: Into<Vec<Number>>,
        T: Into<Vec<Text>>,
    {
        Executable {
            functions: functions.into(),
            numbers: numbers.into(),
            texts: texts.into(),
        }
    }

    pub fn number(&self, index: usize) -> Option<&Number> {
        self.numbers.get(index)
    }

    pub fn text(&self, index: usize) -> Option<&Text> {
        self.texts.get(index)
    }

    pub fn function(&self, index: usize) -> Option<&Function> {
        self.functions.get(index)
    }
}
