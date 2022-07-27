use std::io::{self, Write};

pub type LocalOffset = u8;
pub type CaptureOffset = u8;
pub type ConstantIndex = u8;
pub type FunctionIndex = u8;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Operation {
    Constant(ConstantIndex),
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

impl<W: Write> WriteOperation for W {
    fn write(&mut self, operation: Operation) -> io::Result<usize> {
        match operation {
            Operation::Constant(operand) => {
                let bytes = [0, operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Pop => {
                let bytes = [1];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::GetLocal(operand) => {
                let bytes = [2, operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::GetCapture(operand) => {
                let bytes = [3, operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Equal => {
                let bytes = [4];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Greater => {
                let bytes = [5];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Less => {
                let bytes = [6];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Add => {
                let bytes = [7];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Subtract => {
                let bytes = [8];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Multiply => {
                let bytes = [9];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Divide => {
                let bytes = [10];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Remainder => {
                let bytes = [11];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Call(operand) => {
                let bytes = [12, operand];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Send => {
                let bytes = [13];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Closure(operand, operands) => {
                let bytes = [14, operand];
                self.write_all(&bytes)?;
                self.write_all(&operands[..])?;
                Ok(bytes.len() + operands.len())
            }
            Operation::Return => {
                let bytes = [15];
                self.write_all(&bytes)?;
                Ok(bytes.len())
            }
            Operation::Branch(operand) => {
                let bytes = [16];
                let operand_bytes = u16::to_le_bytes(operand);

                self.write_all(&bytes)?;
                self.write_all(&operand_bytes)?;
                Ok(bytes.len() + operand_bytes.len())
            }
            Operation::BranchIfZero(operand) => {
                let bytes = [17];
                let operand_bytes = u16::to_le_bytes(operand);

                self.write_all(&bytes)?;
                self.write_all(&operand_bytes)?;
                Ok(bytes.len() + operand_bytes.len())
            }
            Operation::BranchIfNonZero(operand) => {
                let bytes = [18];
                let operand_bytes = u16::to_le_bytes(operand);

                self.write_all(&bytes)?;
                self.write_all(&operand_bytes)?;
                Ok(bytes.len() + operand_bytes.len())
            }
        }
    }
}
