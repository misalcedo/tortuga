mod closure;
mod code;
mod courier;
mod error;
mod frame;
mod identifier;
mod operations;
mod value;

pub use code::Code;
pub use courier::Courier;
pub use error::{ErrorKind, RuntimeError};
pub use frame::CallFrame;
pub use identifier::Identifier;
pub use value::Value;

#[derive(Clone, Debug, Default)]
pub struct VirtualMachine<Courier> {
    courier: Courier,
    mailbox: Vec<Value>,
    code: Code,
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
}

type RuntimeResult<T> = Result<T, RuntimeError>;
type OperationResult = RuntimeResult<()>;

impl<C: Courier> VirtualMachine<C> {
    const OPERATIONS_TABLE: [fn(&mut VirtualMachine<C>) -> OperationResult; 2] =
        [VirtualMachine::pop, VirtualMachine::send];

    pub fn process(&mut self) -> Result<Value, RuntimeError> {
        self.frames.push(CallFrame {});
        self.stack.push(Value::Closure);

        let operation = Self::OPERATIONS_TABLE
            .get(0)
            .ok_or_else(|| RuntimeError::from(ErrorKind::UnsupportedOperation(0)))?;

        operation(self)?;

        if self.stack.is_empty() {
            Ok(Value::Closure)
        } else {
            Err(ErrorKind::Crash.into())
        }
    }

    pub fn deliver(&mut self, value: Value) {
        self.mailbox.push(value);
    }

    /// Pops the top of the stack and drops the value.
    pub fn pop(&mut self) -> OperationResult {
        self.stack.pop();

        Ok(())
    }

    /// Sends a message to the identifier at the top of the value stack.
    pub fn send(&mut self) -> OperationResult {
        let identifier = self.pop_identifier()?;
        let message = self.pop_value()?;

        self.courier.deliver(identifier, message);

        Ok(())
    }

    /// Pops a generic value from the stack.
    fn pop_value(&mut self) -> RuntimeResult<Value> {
        self.stack
            .pop()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyStack))
    }

    /// Pops an identifier from the value stack.
    fn pop_identifier(&mut self) -> RuntimeResult<Identifier> {
        self.pop_value()?
            .try_into()
            .map_err(|value| RuntimeError::from(ErrorKind::ExpectedIdentifier(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut machine: VirtualMachine<()> = VirtualMachine::default();

        machine.deliver(Value::default());

        let result = machine.process();

        assert_eq!(result, Ok(Value::Closure))
    }
}
