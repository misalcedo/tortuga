use std::io::{self, Write};

pub type LocalOffset = u8;
pub type CaptureOffset = u8;
pub type ConstantIndex = u8;
pub type FunctionIndex = u8;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Operation {
    ConstantNumber(ConstantIndex),
    ConstantText(ConstantIndex),
    Pop,
    GetLocal(LocalOffset),
    GetCapture(CaptureOffset),
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Not,
    Call(FunctionIndex),
    Send,
    Closure(FunctionIndex, Vec<CaptureOffset>),
    Return,
    Branch(u16),
    BranchIfZero(u16),
    BranchIfNonZero(u16),
}

pub trait WriteOperation {
    fn write(&mut self, operation: Operation) -> io::Result<usize>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
enum OperationCode {
    ConstantNumber,
    ConstantText,
    Pop,
    GetLocal,
    GetCapture,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Not,
    Call,
    Send,
    Closure,
    Return,
    Branch,
    BranchIfZero,
    BranchIfNonZero,
}

impl From<&Operation> for OperationCode {
    fn from(operation: &Operation) -> Self {
        match operation {
            Operation::ConstantNumber(_) => OperationCode::ConstantNumber,
            Operation::ConstantText(_) => OperationCode::ConstantText,
            Operation::Pop => OperationCode::Pop,
            Operation::GetLocal(_) => OperationCode::GetLocal,
            Operation::GetCapture(_) => OperationCode::GetCapture,
            Operation::Equal => OperationCode::Equal,
            Operation::Greater => OperationCode::Greater,
            Operation::Less => OperationCode::Less,
            Operation::Add => OperationCode::Add,
            Operation::Subtract => OperationCode::Subtract,
            Operation::Multiply => OperationCode::Multiply,
            Operation::Divide => OperationCode::Divide,
            Operation::Remainder => OperationCode::Remainder,
            Operation::And => OperationCode::And,
            Operation::Or => OperationCode::Or,
            Operation::Not => OperationCode::Not,
            Operation::Call(_) => OperationCode::Call,
            Operation::Send => OperationCode::Send,
            Operation::Closure(_, _) => OperationCode::Closure,
            Operation::Return => OperationCode::Return,
            Operation::Branch(_) => OperationCode::Branch,
            Operation::BranchIfZero(_) => OperationCode::BranchIfZero,
            Operation::BranchIfNonZero(_) => OperationCode::BranchIfNonZero,
        }
    }
}

impl From<OperationCode> for u8 {
    fn from(code: OperationCode) -> Self {
        code as u8
    }
}

impl From<&Operation> for u8 {
    fn from(operation: &Operation) -> Self {
        OperationCode::from(operation) as u8
    }
}

impl<W: Write> WriteOperation for W {
    fn write(&mut self, operation: Operation) -> io::Result<usize> {
        match &operation {
            Operation::ConstantNumber(operand) => {
                let bytes = [u8::from(&operation), *operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::ConstantText(operand) => {
                let bytes = [u8::from(&operation), *operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Pop => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::GetLocal(operand) => {
                let bytes = [u8::from(&operation), *operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::GetCapture(operand) => {
                let bytes = [u8::from(&operation), *operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Equal => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Greater => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Less => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Add => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Subtract => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Multiply => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Divide => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Remainder => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::And => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Or => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Not => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Call(operand) => {
                let bytes = [u8::from(&operation), *operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Send => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Closure(operand, operands) => {
                let bytes = [u8::from(&operation), *operand];
                self.write_all(&bytes)?;
                self.write_all(&operands[..])?;
                Ok(bytes.len() + operands.len())
            }
            Operation::Return => {
                let bytes = [u8::from(&operation)];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Branch(operand) => {
                let bytes = [u8::from(&operation)];
                let operand_bytes = u16::to_le_bytes(*operand);

                self.write_all(&bytes)?;
                self.write_all(&operand_bytes)?;
                Ok(bytes.len() + operand_bytes.len())
            }
            Operation::BranchIfZero(operand) => {
                let bytes = [u8::from(&operation)];
                let operand_bytes = u16::to_le_bytes(*operand);

                self.write_all(&bytes)?;
                self.write_all(&operand_bytes)?;
                Ok(bytes.len() + operand_bytes.len())
            }
            Operation::BranchIfNonZero(operand) => {
                let bytes = [u8::from(&operation)];
                let operand_bytes = u16::to_le_bytes(*operand);

                self.write_all(&bytes)?;
                self.write_all(&operand_bytes)?;
                Ok(bytes.len() + operand_bytes.len())
            }
        }
    }
}
