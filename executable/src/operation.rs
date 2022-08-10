use crate::Code;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum OperationCode {
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

impl From<&[Operation]> for Code {
    fn from(operations: &[Operation]) -> Code {
        let mut code = Vec::default();

        for operation in operations {
            match operation {
                Operation::ConstantNumber(operand) => {
                    let bytes = [u8::from(operation), *operand];
                    code.extend_from_slice(&bytes);
                }
                Operation::ConstantText(operand) => {
                    let bytes = [u8::from(operation), *operand];
                    code.extend_from_slice(&bytes);
                }
                Operation::Pop => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::GetLocal(operand) => {
                    let bytes = [u8::from(operation), *operand];
                    code.extend_from_slice(&bytes);
                }
                Operation::GetCapture(operand) => {
                    let bytes = [u8::from(operation), *operand];
                    code.extend_from_slice(&bytes);
                }
                Operation::Equal => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Greater => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Less => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Add => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Subtract => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Multiply => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Divide => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Remainder => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::And => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Or => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Not => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Call(operand) => {
                    let bytes = [u8::from(operation), *operand];
                    code.extend_from_slice(&bytes);
                }
                Operation::Send => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Closure(operand, operands) => {
                    let bytes = [u8::from(operation), *operand];
                    code.extend_from_slice(&bytes);
                    code.extend_from_slice(operands);
                }
                Operation::Return => {
                    let bytes = [u8::from(operation)];
                    code.extend_from_slice(&bytes);
                }
                Operation::Branch(operand) => {
                    let bytes = [u8::from(operation)];
                    let operand_bytes = u16::to_le_bytes(*operand);

                    code.extend_from_slice(&bytes);
                    code.extend_from_slice(&operand_bytes);
                }
                Operation::BranchIfZero(operand) => {
                    let bytes = [u8::from(operation)];
                    let operand_bytes = u16::to_le_bytes(*operand);

                    code.extend_from_slice(&bytes);
                    code.extend_from_slice(&operand_bytes);
                }
                Operation::BranchIfNonZero(operand) => {
                    let bytes = [u8::from(operation)];
                    let operand_bytes = u16::to_le_bytes(*operand);

                    code.extend_from_slice(&bytes);
                    code.extend_from_slice(&operand_bytes);
                }
            }
        }

        Code::from(code)
    }
}

impl From<Vec<Operation>> for Code {
    fn from(operations: Vec<Operation>) -> Self {
        operations.as_slice().into()
    }
}
