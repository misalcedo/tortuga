use crate::error::{ErrorKind, RuntimeError};
use crate::Courier;
use crate::Identifier;
use crate::Program;
use crate::Value;
use crate::{CallFrame, Number};
use crate::{Closure, Function};
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

    pub fn process(&mut self, value: Value) -> RuntimeResult<Option<Value>> {
        let function = Function::default();
        let closure = Closure::new(function, Vec::new());

        self.stack.push(closure.into());
        self.stack.push(value);

        self.enter_function(&function)?;

        while self.cursor < self.code.len() {
            let operation = self.get_operation()?;

            operation(self)?;
        }

        self.exit_function(&function)
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
        todo!()
    }

    fn define_local_operation(&mut self) -> OperationResult {
        todo!()
    }

    fn get_capture_operation(&mut self) -> OperationResult {
        todo!()
    }

    fn compare_operation(&mut self) -> OperationResult {
        let value = self.compare()?;

        self.stack.push(value);

        Ok(())
    }

    fn equal_operation(&mut self) -> OperationResult {
        let value = Value::from(self.compare()? == Value::from(0));

        self.stack.push(value);

        Ok(())
    }

    fn greater_operation(&mut self) -> OperationResult {
        let value = Value::from(self.compare()? == Value::from(1));

        self.stack.push(value);

        Ok(())
    }

    fn less_operation(&mut self) -> OperationResult {
        let value = Value::from(self.compare()? == Value::from(-1));

        self.stack.push(value);

        Ok(())
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
        todo!()
    }

    fn send_operation(&mut self) -> OperationResult {
        let identifier = self.pop_identifier()?;
        let message = self.pop_value()?;

        self.courier.deliver(identifier, message);

        Ok(())
    }

    fn closure_operation(&mut self) -> OperationResult {
        todo!()
    }

    fn return_operation(&mut self) -> OperationResult {
        todo!()
    }

    fn branch_operation(&mut self) -> OperationResult {
        let jump = self.read_u32()? as usize;

        self.cursor += jump;

        Ok(())
    }

    fn branch_if_zero_operation(&mut self) -> OperationResult {
        let jump = self.read_u32()? as usize;
        let condition = self.pop_number()? == Number::from(0);

        if condition {
            self.cursor += jump;
        }

        Ok(())
    }

    fn branch_if_non_zero_operation(&mut self) -> OperationResult {
        let jump = self.read_u32()? as usize;
        let condition = self.pop_number()? != Number::from(0);

        if condition {
            self.cursor += jump;
        }

        Ok(())
    }

    fn enter_function(&mut self, function: &Function) -> RuntimeResult<()> {
        let values = function.values();
        let length = self.stack.len();
        let start_stack = length
            .checked_sub(values)
            .ok_or_else(|| RuntimeError::from(ErrorKind::IncorrectCall(values, length)))?;
        let frame = CallFrame::new(self.cursor, start_stack, function.start());

        self.frames.push(frame);

        Ok(())
    }

    fn exit_function(&mut self, function: &Function) -> RuntimeResult<Option<Value>> {
        let values = function.values();
        let frame = self
            .frames
            .last()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyCallFrames))?;
        let result = if self.stack[frame].len() > values {
            self.stack.pop()
        } else {
            None
        };

        for _ in 0..values {
            if self.stack.pop().is_none() {
                return Err(ErrorKind::EmptyStack.into());
            }
        }

        self.frames.pop();

        Ok(result)
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

    fn read_u32(&mut self) -> RuntimeResult<u32> {
        let operand = self.read::<u32>()?;
        let bytes = [operand[0], operand[1], operand[2], operand[3]];

        Ok(u32::from_le_bytes(bytes))
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

    fn compare(&mut self) -> Result<Value, RuntimeError> {
        let b = self.pop_value()?;
        let a = self.pop_value()?;

        match a.partial_cmp(&b) {
            Some(Ordering::Less) => Ok(Value::from(-1)),
            Some(Ordering::Equal) => Ok(Value::from(0)),
            Some(Ordering::Greater) => Ok(Value::from(1)),
            None => Err(ErrorKind::UnsupportedTypes(a, b).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Operations;

    #[test]
    fn add() {
        let code = Program::new(
            vec![
                Operations::Constant as u8,
                0,
                Operations::Constant as u8,
                1,
                Operations::Add as u8,
            ],
            vec![Value::from(1.0), Value::from(2.0)],
            vec![],
        );
        let mut machine: VirtualMachine<()> = VirtualMachine::new(code, ());
        let message = Value::default();
        let result = machine.process(message);

        assert_eq!(result, Ok(Some(Value::from(3.0))));
    }

    #[test]
    fn compare() {
        let code = Program::new(
            vec![
                Operations::Constant as u8,
                0,
                Operations::Constant as u8,
                1,
                Operations::Constant as u8,
                2,
                Operations::Compare as u8,
                Operations::Equal as u8,
            ],
            vec![Value::from(1.0), Value::from(42.0), Value::from(2.0)],
            vec![],
        );
        let mut machine: VirtualMachine<()> = VirtualMachine::new(code, ());
        let message = Value::default();
        let result = machine.process(message);

        assert_eq!(result, Ok(Some(Value::from(true))));
    }

    #[test]
    fn less_and_greater() {
        let code = Program::new(
            vec![
                Operations::Constant as u8,
                0,
                Operations::Constant as u8,
                1,
                Operations::Constant as u8,
                2,
                Operations::Greater as u8,
                Operations::Less as u8,
            ],
            vec![Value::from(2.0), Value::from(42.0), Value::from(1.0)],
            vec![],
        );
        let mut machine: VirtualMachine<()> = VirtualMachine::new(code, ());
        let message = Value::default();
        let result = machine.process(message);

        assert_eq!(result, Ok(Some(Value::from(false))));
    }

    #[test]
    fn unconditional_branch() {
        let code = Program::new(
            vec![
                Operations::Branch as u8,
                2,
                0,
                0,
                0,
                Operations::Constant as u8,
                0,
                Operations::Constant as u8,
                1,
            ],
            vec![Value::from(1.0), Value::from(42.0)],
            vec![],
        );
        let mut machine: VirtualMachine<()> = VirtualMachine::new(code, ());
        let message = Value::default();
        let result = machine.process(message);

        assert_eq!(result, Ok(Some(Value::from(42.0))));
    }

    #[test]
    fn branch_if_zero() {
        let code = Program::new(
            vec![
                Operations::Constant as u8,
                0,
                Operations::BranchIfZero as u8,
                2,
                0,
                0,
                0,
                Operations::Constant as u8,
                1,
                Operations::Constant as u8,
                0,
                Operations::Pop as u8,
            ],
            vec![Value::from(1.0), Value::from(42.0)],
            vec![],
        );
        let mut machine: VirtualMachine<()> = VirtualMachine::new(code, ());
        let message = Value::default();
        let result = machine.process(message);

        assert_eq!(result, Ok(Some(Value::from(42.0))));
    }

    #[test]
    fn branch_if_non_zero() {
        let code = Program::new(
            vec![
                Operations::Constant as u8,
                0,
                Operations::BranchIfNonZero as u8,
                2,
                0,
                0,
                0,
                Operations::Constant as u8,
                0,
                Operations::Constant as u8,
                1,
                Operations::Pop as u8,
            ],
            vec![Value::from(1.0), Value::from(42.0)],
            vec![],
        );
        let mut machine: VirtualMachine<()> = VirtualMachine::new(code, ());
        let message = Value::default();
        let result = machine.process(message);

        assert_eq!(result, Ok(None));
    }
}
