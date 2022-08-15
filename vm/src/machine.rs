use crate::error::{ErrorKind, RuntimeError};
use crate::Closure;
use crate::Executable;
use crate::Identifier;
use crate::Value;
use crate::{CallFrame, Number};
use crate::{Courier, Text};
use std::cmp::Ordering;
use std::mem;

#[derive(Clone, Debug)]
pub struct VirtualMachine<Courier> {
    courier: Courier,
    executable: Executable,
    stack: Vec<Value>,
    frame: CallFrame,
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
    const OPERATIONS_TABLE: [fn(&mut VirtualMachine<C>) -> OperationResult; 29] = [
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
        Self::group_operation,
        Self::separate_operation,
    ];

    pub fn new<E>(executable: E, courier: C) -> Self
    where
        E: Into<Executable>,
    {
        VirtualMachine {
            courier,
            executable: executable.into(),
            frame: CallFrame::default(),
            frames: vec![],
            stack: vec![],
        }
    }

    pub fn set_executable<E>(&mut self, executable: E)
    where
        E: Into<Executable>,
    {
        self.executable = executable.into();
    }

    pub fn call(&mut self, index: usize, arguments: &[Value]) -> RuntimeResult<Option<Value>> {
        let closure = Closure::new(index, vec![]);

        self.stack.push(closure.into());
        self.stack.extend_from_slice(arguments);

        self.enter_function(index)?;

        while let Some(index) = self.frame.next() {
            let operation = self.get_operation(index as usize)?;

            operation(self)?;
        }

        if self.frames.is_empty() {
            Ok(self.stack.pop())
        } else {
            Err(ErrorKind::FunctionMissingReturn(index).into())
        }
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
        if self.stack[self.frame.temporaries()].is_empty() || self.stack.pop().is_none() {
            Err(ErrorKind::EmptyStack.into())
        } else {
            Ok(())
        }
    }

    fn define_local_operation(&mut self) -> OperationResult {
        let slot = self
            .frame
            .define_local()
            .map_err(|locals| RuntimeError::from(ErrorKind::TooManyLocals(locals)))?;

        self.set_local(slot)?;

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

        self.stack.push(value.inner());

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

        self.stack.push(value.inner());

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
        let slot = self.read_byte()? as usize;

        self.enter_function(slot)?;

        Ok(())
    }

    fn send_operation(&mut self) -> OperationResult {
        let identifier = self.pop_identifier()?;
        let message = self.pop_value()?;

        self.courier.deliver(identifier, message);

        Ok(())
    }

    fn closure_operation(&mut self) -> OperationResult {
        let slot = self.read_byte()? as usize;
        let function = self
            .executable
            .function(slot)
            .ok_or_else(|| RuntimeError::from(ErrorKind::NoSuchFunction(slot)))?;

        let mut captures = Vec::with_capacity(function.captures().len());
        for &local in function.captures().iter() {
            let index = self.read_byte()? as usize;
            let value = if local {
                self.capture_local(index)?
            } else {
                self.get_capture(index)?
            };

            captures.push(value);
        }

        self.stack.push(Closure::new(slot, captures).into());

        Ok(())
    }

    fn return_operation(&mut self) -> OperationResult {
        if let Some(result) = self.exit_function()? {
            self.stack.push(result);
        }

        Ok(())
    }

    fn branch_operation(&mut self) -> OperationResult {
        let jump = self.read_u16()? as usize;

        self.frame.jump(jump);

        Ok(())
    }

    fn branch_if_zero_operation(&mut self) -> OperationResult {
        let jump = self.read_u16()? as usize;
        let condition = self.pop_number()? == Number::from(0);

        if condition {
            self.frame.jump(jump);
        }

        Ok(())
    }

    fn branch_if_non_zero_operation(&mut self) -> OperationResult {
        let jump = self.read_u16()? as usize;
        let condition = self.pop_number()? != Number::from(0);

        if condition {
            self.frame.jump(jump);
        }

        Ok(())
    }

    fn group_operation(&mut self) -> OperationResult {
        let parts = self.read_byte()? as usize;
        let mut inner = Vec::with_capacity(parts);

        for _ in 0..parts {
            inner.push(self.pop_value()?);
        }

        self.stack.push(Value::group(inner));

        Ok(())
    }

    fn separate_operation(&mut self) -> OperationResult {
        let grouping = self.pop_value()?;

        for part in grouping.iter() {
            self.stack.push(part.clone());
        }

        Ok(())
    }

    fn enter_function(&mut self, index: usize) -> RuntimeResult<()> {
        let function = self
            .executable
            .function(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::NoSuchFunction(index)))?;

        let parameters = 1 + function.arity();
        let start_stack = self.stack.len().checked_sub(parameters).ok_or_else(|| {
            RuntimeError::from(ErrorKind::NotEnoughParameters(parameters, self.stack.len()))
        })?;
        let mut frame = CallFrame::new(start_stack, function);

        mem::swap(&mut frame, &mut self.frame);

        self.frames.push(frame);

        for _ in 0..function.locals() {
            self.stack.push(Value::default());
        }

        if !function.captures().is_empty() {
            let range = self.frame.locals();
            let locals = range.len();
            let value = self.stack[range]
                .get(0)
                .ok_or_else(|| RuntimeError::from(ErrorKind::UndefinedLocal(0, locals)))?;
            let captures = match value {
                Value::Closure(closure) => closure.captures(),
                _ => return Err(ErrorKind::ExpectedClosure(value.clone()).into()),
            };

            self.stack.extend_from_slice(&captures)
        }

        Ok(())
    }

    fn exit_function(&mut self) -> RuntimeResult<Option<Value>> {
        let result = self.stack.pop();

        self.stack.drain(self.frame.all());

        let mut frame = self
            .frames
            .pop()
            .ok_or_else(|| RuntimeError::from(ErrorKind::EmptyCallFrames))?;

        mem::swap(&mut frame, &mut self.frame);

        Ok(result)
    }

    fn get_operation(
        &mut self,
        operation: usize,
    ) -> RuntimeResult<&fn(&mut VirtualMachine<C>) -> OperationResult> {
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

    fn set_local(&mut self, index: usize) -> RuntimeResult<()> {
        let value = self.pop_value()?;
        let range = self.frame.locals();
        let locals = range.len();
        let local = self.stack[range]
            .get_mut(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::UndefinedLocal(index, locals)))?;

        local.update(value);

        Ok(())
    }

    fn capture_local(&mut self, index: usize) -> RuntimeResult<Value> {
        let range = self.frame.locals();
        let locals = range.len();
        let local = self.stack[range]
            .get_mut(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::UndefinedLocal(index, locals)))?;

        Ok(local.capture())
    }

    fn get_local(&mut self, index: usize) -> RuntimeResult<Value> {
        let range = self.frame.locals();
        let locals = range.len();
        let value = self.stack[range]
            .get(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::UndefinedLocal(index, locals)))?;

        Ok(value.clone())
    }

    fn set_capture(&mut self, index: usize) -> RuntimeResult<()> {
        let value = self.pop_value()?;
        let range = self.frame.captures();
        let captures = range.len();
        let capture = self.stack[range]
            .get_mut(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::UndefinedCapture(index, captures)))?;

        capture.update(value);

        Ok(())
    }

    fn get_capture(&mut self, index: usize) -> RuntimeResult<Value> {
        let range = self.frame.captures();
        let captures = range.len();
        let value = self.stack[range]
            .get(index)
            .ok_or_else(|| RuntimeError::from(ErrorKind::UndefinedCapture(index, captures)))?;

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

    fn read_byte(&mut self) -> RuntimeResult<u8> {
        self.frame
            .read_byte()
            .ok_or_else(|| RuntimeError::from(ErrorKind::InvalidOperand(mem::size_of::<u8>(), 0)))
    }

    fn read_u16(&mut self) -> RuntimeResult<u16> {
        self.frame.read_u16().map_err(|actual| {
            RuntimeError::from(ErrorKind::InvalidOperand(mem::size_of::<u16>(), actual))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tortuga_executable::{Function, Operation, ToCode};

    #[test]
    fn add_with_local() {
        let operations = vec![
            Operation::ConstantNumber(0),
            Operation::DefineLocal,
            Operation::GetLocal(1),
            Operation::ConstantNumber(1),
            Operation::Add,
            Operation::Return,
        ];
        let main = Function::new(0, 1, operations.to_code(), vec![]);
        let executable = Executable::new(vec![main], vec![2.into(), 40.into()], vec![]);
        let mut machine = VirtualMachine::new(executable, ());

        assert_eq!(machine.call(0, &[]), Ok(Some(Value::from(42))));
    }
}
