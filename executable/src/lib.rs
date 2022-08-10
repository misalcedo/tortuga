extern crate core;

mod code;
mod error;
mod function;
mod number;
mod operation;
mod text;

pub use code::Code;
pub use error::ParseNumberError;
pub use function::Function;
pub use number::Number;
pub use operation::{Operation, OperationCode};
pub use text::Text;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Executable {
    code: Vec<u8>,
    functions: Vec<Function>,
    numbers: Vec<Number>,
    texts: Vec<Text>,
}

impl Executable {
    pub fn new<C, F, N, T>(code: C, functions: F, numbers: N, texts: T) -> Self
    where
        C: Into<Code>,
        F: Into<Vec<Function>>,
        N: Into<Vec<Number>>,
        T: Into<Vec<Text>>,
    {
        Executable {
            code: Vec::from(code.into()),
            functions: functions.into(),
            numbers: numbers.into(),
            texts: texts.into(),
        }
    }

    pub fn code(&self, offset: usize, size: usize) -> &[u8] {
        &self.code[offset..offset + size]
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

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }
}
