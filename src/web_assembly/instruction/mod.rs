use crate::web_assembly::{
    Expression, FunctionIndex, Identifier, TableIndex, TypeIndex, TypeUse, ValueType,
};

pub struct LabelIndex(Identifier);

/// Instructions are syntactically distinguished into plain and structured instructions.
pub enum Instruction {
    // Blocks
    Block {
        label: Identifier,
        expression: Expression,
        signature: BlockType,
    },
    Loop {
        label: Identifier,
        expression: Expression,
        signature: BlockType,
    },
    If {
        label: Identifier,
        positive: Expression,
        negative: Expression,
        signature: BlockType,
    },
    // Control
    Unreachable,
    Nop,
    Branch(LabelIndex),
    BranchIf(LabelIndex),
    BranchTable(Vec<LabelIndex>, LabelIndex),
    Return,
    Call(FunctionIndex),
    CallIndirect(TableIndex, TypeUse),
    // Numerical
}

pub struct BlockType {
    signature: TypeIndex,
    value_type: ValueType,
}

pub enum ConstantInstruction {}
