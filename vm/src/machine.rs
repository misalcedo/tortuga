use crate::error::{ErrorKind, RuntimeError};
use crate::Closure;
use crate::Courier;
use crate::Identifier;
use crate::Program;
use crate::Value;
use crate::{CallFrame, Number};
use std::cmp::Ordering;
use std::mem::size_of;

#[derive(Clone, Debug, Default)]
pub struct VirtualMachine<Courier> {
    courier: Courier,
    code: Program,
    cursor: usize,
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
}

type RuntimeResult<T> = Result<T, RuntimeError>;
type OperationResult = RuntimeResult<()>;

impl<C: Courier> VirtualMachine<C> {
    const OPERATIONS_TABLE: [fn(&mut VirtualMachine<C>) -> OperationResult; 21] = [
        Self::constant_operation,
        Self::pop_operation,
        Self::get_local_operation,
        Self::define_local_operation,
        Self::get_capture_operation,
        Self::compare_operation,
        Self::equal_operation,
        Self::greater_operation,
        Self::less_operation,
        Self::add_operation,
        Self::subtract_operation,
        Self::multiply_operation,
        Self::divide_operation,
        Self::remainder_operation,
        Self::call_operation,
        Self::send_operation,
        Self::closure_operation,
        Self::return_operation,
        Self::branch_operation,
        Self::branch_if_zero_operation,
        Self::branch_if_non_zero_operation,
    ];

    pub fn new(code: Program, courier: C) -> Self {
        VirtualMachine {
            courier,
            code,
            cursor: 0,
            stack: vec![],
            frames: vec![],
        }
    }

    pub fn process(&mut self, value: Value) -> Result<Option<Value>, RuntimeError> {
        self.frames.push(CallFrame::default());
        self.stack.push(Closure::default().into());
        self.stack.push(value);

        while self.cursor < self.code.len() {
            let operation = self.get_operation()?;

            operation(self)?;
        }

        Ok(self.stack.pop())
    }

    fn constant_operation(&mut self) -> OperationResult {
        let constant = self.get_constant()?.clone();

        self.stack.push(constant);

        Ok(())
    }

    fn pop_operation(&mut self) -> OperationResult {
        match self.stack.pop() {
            Some(_) => Ok(()),
            None => Err(ErrorKind::EmptyStack.into()),
        }
    }

    fn get_local_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn define_local_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn get_capture_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn compare_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;

        match a.partial_cmp(&b) {
            Some(Ordering::Less) => Ok(self.stack.push(Value::from(Number::from(-1)))),
            Some(Ordering::Equal) => Ok(self.stack.push(Value::from(Number::from(0)))),
            Some(Ordering::Greater) => Ok(self.stack.push(Value::from(Number::from(1)))),
            None => Err(ErrorKind::UnsupportedTypes(a, b).into()),
        }
    }

    fn equal_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;

        match a.partial_cmp(&b) {
            Some(Ordering::Equal) => Ok(self.stack.push(Value::from(Number::from(1)))),
            Some(_) => Ok(self.stack.push(Value::from(Number::from(0)))),
            None => Err(ErrorKind::UnsupportedTypes(a, b).into()),
        }
    }

    fn greater_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;

        match a.partial_cmp(&b) {
            Some(Ordering::Greater) => Ok(self.stack.push(Value::from(Number::from(1)))),
            Some(_) => Ok(self.stack.push(Value::from(Number::from(0)))),
            None => Err(ErrorKind::UnsupportedTypes(a, b).into()),
        }
    }

    fn less_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;

        match a.partial_cmp(&b) {
            Some(Ordering::Less) => Ok(self.stack.push(Value::from(Number::from(1)))),
            Some(_) => Ok(self.stack.push(Value::from(Number::from(0)))),
            None => Err(ErrorKind::UnsupportedTypes(a, b).into()),
        }
    }

    fn add_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = a + b;
        let value = result
            .map_err(|(lhs, rhs)| RuntimeError::from(ErrorKind::UnsupportedTypes(lhs, rhs)))?;

        self.stack.push(value);

        Ok(())
    }

    fn subtract_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = a - b;
        let value = result
            .map_err(|(lhs, rhs)| RuntimeError::from(ErrorKind::UnsupportedTypes(lhs, rhs)))?;

        self.stack.push(value);

        Ok(())
    }

    fn multiply_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = a * b;
        let value = result
            .map_err(|(lhs, rhs)| RuntimeError::from(ErrorKind::UnsupportedTypes(lhs, rhs)))?;

        self.stack.push(value);

        Ok(())
    }
    fn divide_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = a / b;
        let value = result
            .map_err(|(lhs, rhs)| RuntimeError::from(ErrorKind::UnsupportedTypes(lhs, rhs)))?;

        self.stack.push(value);

        Ok(())
    }

    fn remainder_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = a % b;
        let value = result
            .map_err(|(lhs, rhs)| RuntimeError::from(ErrorKind::UnsupportedTypes(lhs, rhs)))?;

        self.stack.push(value);

        Ok(())
    }

    fn call_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn send_operation(&mut self) -> OperationResult {
        let identifier = self.pop_identifier()?;
        let message = self.pop_value()?;

        self.courier.deliver(identifier, message);

        Ok(())
    }

    fn closure_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn return_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn branch_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn branch_if_zero_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn branch_if_non_zero_operation(&mut self) -> OperationResult {
        Ok(())
    }

    fn get_operation(&mut self) -> RuntimeResult<&fn(&mut VirtualMachine<C>) -> OperationResult> {
        let operation = self.read_byte()? as usize;

        Self::OPERATIONS_TABLE
            .get(operation)
            .ok_or_else(|| ErrorKind::UnsupportedOperation(operation).into())
    }

    fn get_constant(&mut self) -> RuntimeResult<&Value> {
        let index = self.read_byte()? as usize;

        self.code
            .constant(index)
            .ok_or_else(|| ErrorKind::NoSuchConstant(index).into())
    }

    fn read_byte(&mut self) -> RuntimeResult<u8> {
        Ok(self.read::<u8>()?[0])
    }

    fn read<T>(&mut self) -> RuntimeResult<&[u8]> {
        let size = size_of::<T>();
        let operand = self.code.content(self.cursor, size);

        if operand.len() == size {
            self.cursor += size;

            Ok(operand)
        } else {
            Err(ErrorKind::InvalidOperand(size, operand.len()).into())
        }
    }

    fn pop_value(&mut self) -> RuntimeResult<Value> {
        self.stack
            .pop()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyStack))
    }

    fn pop_identifier(&mut self) -> RuntimeResult<Identifier> {
        self.pop_value()?
            .try_into()
            .map_err(|value| ErrorKind::ExpectedIdentifier(value).into())
    }

    fn pop_number(&mut self) -> RuntimeResult<Number> {
        self.pop_value()?
            .try_into()
            .map_err(|value| ErrorKind::ExpectedNumber(value).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Number, Operations};

    #[test]
    fn empty() {
        let code = Program::new(
            vec![
                Operations::Constant as u8,
                0,
                Operations::Constant as u8,
                1,
                Operations::Add as u8,
            ],
            vec![Number::from(1.0).into(), Number::from(2.0).into()],
        );
        let mut machine: VirtualMachine<()> = VirtualMachine::new(code, ());
        let message = Value::default();
        let result = machine.process(message);

        assert_eq!(result, Ok(Some(Value::from(Number::from(3.0)))));
    }
}