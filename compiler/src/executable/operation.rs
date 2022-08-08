use std::io::{self, Write};

pub type LocalOffset = u8;
pub type CaptureOffset = u8;
pub type ConstantIndex = u8;
pub type FunctionIndex = u8;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Operation {
    ConstantNumber(ConstantIndex),
    ConstantUri(ConstantIndex),
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
enum OperationIndex {
    ConstantNumber,
    ConstantUri,
    ConstantFunction,
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

impl From<&Operation> for u8 {
    fn from(operation: &Operation) -> Self {
        match operation {
            Operation::ConstantNumber(_) => OperationIndex::ConstantNumber as u8,
            Operation::ConstantUri(_) => OperationIndex::ConstantUri as u8,
            Operation::ConstantFunction(_) => OperationIndex::ConstantFunction as u8,
            Operation::Pop => OperationIndex::Pop as u8,
            Operation::GetLocal(_) => OperationIndex::GetLocal as u8,
            Operation::GetCapture(_) => OperationIndex::GetCapture as u8,
            Operation::Equal => OperationIndex::Equal as u8,
            Operation::Greater => OperationIndex::Greater as u8,
            Operation::Less => OperationIndex::Less as u8,
            Operation::Add => OperationIndex::Add as u8,
            Operation::Subtract => OperationIndex::Subtract as u8,
            Operation::Multiply => OperationIndex::Multiply as u8,
            Operation::Divide => OperationIndex::Divide as u8,
            Operation::Remainder => OperationIndex::Remainder as u8,
            Operation::And => OperationIndex::And as u8,
            Operation::Or => OperationIndex::Or as u8,
            Operation::Not => OperationIndex::Not as u8,
            Operation::Call(_) => OperationIndex::Call as u8,
            Operation::Send => OperationIndex::Send as u8,
            Operation::Closure(_, _) => OperationIndex::Closure as u8,
            Operation::Return => OperationIndex::Return as u8,
            Operation::Branch(_) => OperationIndex::Branch as u8,
            Operation::BranchIfZero(_) => OperationIndex::BranchIfZero as u8,
            Operation::BranchIfNonZero(_) => OperationIndex::BranchIfNonZero as u8,
        }
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
            Operation::ConstantUri(operand) => {
                let bytes = [u8::from(&operation), *operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::ConstantFunction(operand) => {
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
