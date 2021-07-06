use crate::web_assembly::{ConstantInstruction, Instruction};

pub struct Expression {
    instructions: Vec<Instruction>,
}

pub struct ConstantExpression {
    instructions: Vec<ConstantInstruction>,
}
