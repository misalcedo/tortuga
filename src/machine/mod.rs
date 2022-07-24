mod closure;
mod code;
mod error;
mod frame;
mod operations;
mod value;

pub use code::Code;
pub use frame::CallFrame;
pub use value::Value;
use crate::machine::error::RuntimeError;

#[derive(Clone, Debug, Default)]
pub struct VirtualMachine {
    stack: Vec<Value>,
    frames: Vec<CallFrame>
}

impl VirtualMachine {
    pub fn run(&mut self, _code: Code) -> Result<Value, RuntimeError> {
        Err(RuntimeError {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let result = VirtualMachine::default().run(Code::default());

        assert_eq!(result, Err(RuntimeError {}))
    }
}