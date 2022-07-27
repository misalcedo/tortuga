pub type LocalOffset = u8;
pub type CaptureOffset = u8;
pub type ConstantIndex = u8;
pub type FunctionIndex = u8;

pub enum Operations {
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
    Branch,
    BranchIfZero,
    BranchIfNonZero,
}
