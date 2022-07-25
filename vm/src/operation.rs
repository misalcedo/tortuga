#[repr(u8)]
pub enum Operations {
    Constant,
    Pop,
    GetLocal,
    DefineLocal,
    GetCapture,
    Compare,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Call,
    Send,
    Closure,
    Return,
    Branch,
    BranchIfZero,
    BranchIfNonZero,
}
