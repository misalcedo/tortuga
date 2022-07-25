use crate::error::{ErrorKind, RuntimeError};
use crate::CallFrame;
use crate::Closure;
use crate::Courier;
use crate::Identifier;
use crate::Program;
use crate::Value;

#[derive(Clone, Debug, Default)]
pub struct VirtualMachine<Courier> {
    courier: Courier,
    mailbox: Vec<Value>,
    code: Program,
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
        self.stack.push(Value::default());
        self.stack.push(Value::Identifier(Identifier::default()));
        self.stack.push(Value::Closure(Closure::default()));

        let operation = self.get_operation(0)?;

        operation(self)?;

        let operation = self.get_operation(1)?;

        operation(self)?;

        if self.stack.is_empty() {
            Ok(Value::Closure(Closure::default()))
        } else {
            Err(ErrorKind::Crash.into())
        }
    }

    fn get_operation(
        &self,
        index: usize,
    ) -> RuntimeResult<&fn(&mut VirtualMachine<C>) -> OperationResult> {
        Self::OPERATIONS_TABLE
            .get(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::UnsupportedOperation(index)))
    }

    pub fn deliver(&mut self, value: Value) {
        self.mailbox.push(value);
    }

    pub fn pop(&mut self) -> OperationResult {
        self.stack.pop();

        Ok(())
    }

    pub fn send(&mut self) -> OperationResult {
        let identifier = self.pop_identifier()?;
        let message = self.pop_value()?;

        self.courier.deliver(identifier, message);

        Ok(())
    }

    fn pop_value(&mut self) -> RuntimeResult<Value> {
        self.stack
            .pop()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyStack))
    }

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

        assert_eq!(result, Ok(Value::Closure(Closure::default())))
    }
}
