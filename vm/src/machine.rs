use crate::error::{ErrorKind, RuntimeError};
use crate::Executable;
use crate::Identifier;
use crate::Value;
use crate::{CallFrame, Number};
use crate::{Closure, Function};
use crate::{Courier, Text};
use std::cmp::Ordering;
use std::mem::size_of;

#[derive(Clone, Debug)]
pub struct VirtualMachine<Courier> {
    courier: Courier,
    executable: Executable,
    cursor: usize,
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
}

impl Default for VirtualMachine<()> {
    fn default() -> Self {
        VirtualMachine::new(Executable::default(), ())
    }
}

type RuntimeResult<T> = Result<T, RuntimeError>;
type OperationResult = RuntimeResult<()>;

impl<C: Courier> VirtualMachine<C> {
    const OPERATIONS_TABLE: [fn(&mut VirtualMachine<C>) -> OperationResult; 27] = [
        Self::constant_number_operation,
        Self::constant_text_operation,
        Self::pop_operation,
        Self::define_local_operation,
        Self::set_local_operation,
        Self::get_local_operation,
        Self::set_capture_operation,
        Self::get_capture_operation,
        Self::equal_operation,
        Self::greater_operation,
        Self::less_operation,
        Self::add_operation,
        Self::subtract_operation,
        Self::multiply_operation,
        Self::divide_operation,
        Self::remainder_operation,
        Self::power_operation,
        Self::and_operation,
        Self::or_operation,
        Self::not_operation,
        Self::call_operation,
        Self::send_operation,
        Self::closure_operation,
        Self::return_operation,
        Self::branch_operation,
        Self::branch_if_zero_operation,
        Self::branch_if_non_zero_operation,
    ];

    pub fn new<E>(executable: E, courier: C) -> Self
    where
        E: Into<Executable>,
    {
        VirtualMachine {
            courier,
            executable: executable.into(),
            cursor: 0,
            stack: vec![],
            frames: vec![],
        }
    }

    pub fn set_executable<E>(&mut self, executable: E)
    where
        E: Into<Executable>,
    {
        self.executable = executable.into();
    }

    pub fn respond(&mut self, message: Value) -> RuntimeResult<Option<Value>> {
        self.execute(Some(message))?;
        self.exit_function()
    }

    fn execute(&mut self, value: Option<Value>) -> RuntimeResult<()> {
        let locals = if value.is_some() { 1 } else { 0 };
        let function = Function::new(self.cursor, locals, Vec::default());
        let closure = Closure::new(function.clone(), Vec::new());

        self.stack.push(closure.into());

        if let Some(value) = value {
            self.stack.push(value);
        }

        self.enter_function(function)?;

        while self.cursor < self.executable.len() {
            let operation = self.get_operation()?;

            operation(self)?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> RuntimeResult<Option<Value>> {
        self.execute(None)?;

        let cursor = self.cursor;
        let result = self.exit_function()?;

        self.cursor = cursor;

        Ok(result)
    }

    fn constant_number_operation(&mut self) -> OperationResult {
        let constant = self.get_constant_number()?.clone();

        self.stack.push(constant.into());

        Ok(())
    }

    fn constant_text_operation(&mut self) -> OperationResult {
        let constant = self.get_constant_text()?.clone();

        self.stack.push(constant.into());

        Ok(())
    }

    fn pop_operation(&mut self) -> OperationResult {
        match self.stack.pop() {
            Some(_) => Ok(()),
            None => Err(ErrorKind::EmptyStack.into()),
        }
    }

    fn define_local_operation(&mut self) -> OperationResult {
        self.set_local(todo!())?;

        Ok(())
    }

    fn set_local_operation(&mut self) -> OperationResult {
        let slot = self.read_byte()? as usize;

        self.set_local(slot)?;

        Ok(())
    }

    fn get_local_operation(&mut self) -> OperationResult {
        let slot = self.read_byte()? as usize;
        let value = self.get_local(slot)?;

        self.stack.push(value);

        Ok(())
    }

    fn set_capture_operation(&mut self) -> OperationResult {
        let slot = self.read_byte()? as usize;
        self.set_capture(slot)?;

        Ok(())
    }

    fn get_capture_operation(&mut self) -> OperationResult {
        let slot = self.read_byte()? as usize;
        let value = self.get_capture(slot)?;

        self.stack.push(value);

        Ok(())
    }

    fn equal_operation(&mut self) -> OperationResult {
        let value = Value::from(self.compare()? == 0);

        self.stack.push(value);

        Ok(())
    }

    fn greater_operation(&mut self) -> OperationResult {
        let value = Value::from(self.compare()? == 1);

        self.stack.push(value);

        Ok(())
    }

    fn less_operation(&mut self) -> OperationResult {
        let value = Value::from(self.compare()? == -1);

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

    fn power_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = a ^ b;
        let value = result
            .map_err(|(lhs, rhs)| RuntimeError::from(ErrorKind::UnsupportedTypes(lhs, rhs)))?;

        self.stack.push(value);

        Ok(())
    }

    fn and_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = a & b;
        let value = result
            .map_err(|(lhs, rhs)| RuntimeError::from(ErrorKind::UnsupportedTypes(lhs, rhs)))?;

        self.stack.push(value);

        Ok(())
    }

    fn or_operation(&mut self) -> OperationResult {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = a | b;
        let value = result
            .map_err(|(lhs, rhs)| RuntimeError::from(ErrorKind::UnsupportedTypes(lhs, rhs)))?;

        self.stack.push(value);

        Ok(())
    }

    fn not_operation(&mut self) -> OperationResult {
        let a = self.pop_value()?;
        let result = !a;
        let value = result.map_err(|v| RuntimeError::from(ErrorKind::UnsupportedType(v)))?;

        self.stack.push(value);

        Ok(())
    }

    fn call_operation(&mut self) -> OperationResult {
        let function = self.get_function()?;

        self.enter_function(function)?;

        Ok(())
    }

    fn send_operation(&mut self) -> OperationResult {
        let identifier = self.pop_identifier()?;
        let message = self.pop_value()?;

        self.courier.deliver(identifier, message);

        Ok(())
    }

    fn closure_operation(&mut self) -> OperationResult {
        let function = self.get_function()?;

        let mut captures = Vec::with_capacity(function.captures().len());
        for local in function.captures() {
            let index = self.read_byte()? as usize;
            let value = if *local {
                self.get_local(index)?
            } else {
                self.get_capture(index)?
            };

            captures.push(value);
        }

        self.stack.push(Closure::new(function, captures).into());

        Ok(())
    }

    fn return_operation(&mut self) -> OperationResult {
        if let Some(result) = self.exit_function()? {
            self.stack.push(result);
        }

        if self.frames.is_empty() {
            Err(ErrorKind::ReturnOutsideFunction.into())
        } else {
            Ok(())
        }
    }

    fn branch_operation(&mut self) -> OperationResult {
        let jump = self.read_u16()? as usize;

        self.cursor += jump;

        Ok(())
    }

    fn branch_if_zero_operation(&mut self) -> OperationResult {
        let jump = self.read_u16()? as usize;
        let condition = self.pop_number()? == Number::from(0);

        if condition {
            self.cursor += jump;
        }

        Ok(())
    }

    fn branch_if_non_zero_operation(&mut self) -> OperationResult {
        let jump = self.read_u16()? as usize;
        let condition = self.pop_number()? != Number::from(0);

        if condition {
            self.cursor += jump;
        }

        Ok(())
    }

    fn enter_function(&mut self, function: Function) -> RuntimeResult<()> {
        let locals = function.locals() + 1;
        let has_captures = function.captures().len() > 0;
        let length = self.stack.len();
        let start_stack = length
            .checked_sub(locals)
            .ok_or_else(|| RuntimeError::from(ErrorKind::IncorrectCall(locals, length)))?;
        let start_frame = function.start();
        let frame = CallFrame::new(self.cursor, start_stack, function);

        self.frames.push(frame);

        if has_captures {
            let closure = self.get_current_closure()?;
            let captures = Vec::from(closure);

            self.stack.extend(captures);
        }

        self.cursor = start_frame;

        Ok(())
    }

    fn exit_function(&mut self) -> RuntimeResult<Option<Value>> {
        let frame = self
            .frames
            .pop()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyCallFrames))?;
        let values = frame.values();
        let result = if self.stack[&frame].len() > values {
            self.stack.pop()
        } else {
            None
        };

        for _ in 0..values {
            if self.stack.pop().is_none() {
                return Err(ErrorKind::EmptyStack.into());
            }
        }

        self.cursor = frame.return_to();

        Ok(result)
    }

    fn get_operation(&mut self) -> RuntimeResult<&fn(&mut VirtualMachine<C>) -> OperationResult> {
        let operation = self.read_byte()? as usize;

        Self::OPERATIONS_TABLE
            .get(operation)
            .ok_or_else(|| ErrorKind::UnsupportedOperation(operation).into())
    }

    fn get_constant_number(&mut self) -> RuntimeResult<&Number> {
        let index = self.read_byte()? as usize;

        self.executable
            .number(index)
            .ok_or_else(|| ErrorKind::NoSuchConstant(index).into())
    }

    fn get_constant_text(&mut self) -> RuntimeResult<&Text> {
        let index = self.read_byte()? as usize;

        self.executable
            .text(index)
            .ok_or_else(|| ErrorKind::NoSuchConstant(index).into())
    }

    fn read_byte(&mut self) -> RuntimeResult<u8> {
        Ok(self.read::<u8>()?[0])
    }

    fn read_u16(&mut self) -> RuntimeResult<u16> {
        let operand = self.read::<u16>()?;
        let bytes = [operand[0], operand[1]];

        Ok(u16::from_le_bytes(bytes))
    }

    fn read<T>(&mut self) -> RuntimeResult<&[u8]> {
        let size = size_of::<T>();
        let operand = self.executable.code(self.cursor, size);

        if operand.len() == size {
            self.cursor += size;

            Ok(operand)
        } else {
            Err(ErrorKind::InvalidOperand(size, operand.len()).into())
        }
    }

    fn set_local(&mut self, index: usize) -> RuntimeResult<()> {
        let value = self.pop_value()?;
        let frame = self
            .frames
            .last()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyCallFrames))?;
        let local = self.stack[frame]
            .get_mut(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::CorruptedFrame))?;

        *local = value;

        Ok(())
    }

    fn get_local(&mut self, index: usize) -> RuntimeResult<Value> {
        let frame = self
            .frames
            .last()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyCallFrames))?;
        let value = self.stack[frame]
            .get(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::CorruptedFrame))?;

        Ok(value.clone())
    }

    fn set_capture(&mut self, index: usize) -> RuntimeResult<()> {
        let value = self.pop_value()?;
        let frame = self
            .frames
            .last()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyCallFrames))?;
        let capture = self.stack[frame]
            .get_mut(frame.locals() + index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::CorruptedFrame))?;

        *capture = value;

        Ok(())
    }

    fn get_capture(&mut self, index: usize) -> RuntimeResult<Value> {
        let frame = self
            .frames
            .last()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyCallFrames))?;
        let value = self.stack[frame]
            .get(frame.locals() + index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::CorruptedFrame))?;

        Ok(value.clone())
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

    fn compare(&mut self) -> Result<i8, RuntimeError> {
        let b = self.pop_value()?;
        let a = self.pop_value()?;

        match a.partial_cmp(&b) {
            Some(Ordering::Less) => Ok(-1),
            Some(Ordering::Equal) => Ok(0),
            Some(Ordering::Greater) => Ok(1),
            None => Err(ErrorKind::UnsupportedTypes(a, b).into()),
        }
    }

    fn get_function(&mut self) -> RuntimeResult<Function> {
        let slot = self.read_byte()? as usize;
        let function = self
            .executable
            .function(slot)
            .ok_or_else(|| RuntimeError::from(ErrorKind::NoSuchFunction(slot)))?;

        Ok(function.clone())
    }

    fn get_current_closure(&mut self) -> RuntimeResult<Closure> {
        self.get_local(0)?
            .try_into()
            .map_err(|value| ErrorKind::ExpectedClosure(value).into())
    }
}
