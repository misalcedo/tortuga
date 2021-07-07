use crate::web_assembly::{
    DataIndex, ElementIndex, Expression, FunctionIndex, GlobalIndex, Identifier, LocalIndex,
    NumberType, TableIndex, TypeIndex, TypeUse, ValueType,
};

pub struct LabelIndex(Identifier);

/// Instructions are syntactically distinguished into plain and structured instructions.
pub enum Instruction {
    // Numeric
    // Reference
    // Parametric
    // Variable
    LocalGet(LocalIndex),
    LocalSet(LocalIndex),
    LocalTee(LocalIndex),
    GlobalGet(GlobalIndex),
    GlobalSet(GlobalIndex),
    // Table
    TableGet(TableIndex),
    TableSet(TableIndex),
    TableSize(TableIndex),
    TableGrow(TableIndex),
    TableFill(TableIndex),
    TableCopy(TableIndex, TableIndex),
    TableInit(TableIndex, ElementIndex),
    ElementDrop(ElementIndex),
    // Memory
    Load(NumberType, MemoryArgument),
    Store(NumberType, MemoryArgument),
    LoadPartial(StorageSize, SignExtension, MemoryArgument),
    StorePartial(StorageSize, MemoryArgument),
    MemorySize,
    MemoryGrow,
    MemoryFill,
    MemoryCopy,
    MemoryInit(DataIndex),
    DatDrop(DataIndex),
    // Control
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
    Unreachable,
    Nop,
    Branch(LabelIndex),
    BranchIf(LabelIndex),
    BranchTable(Vec<LabelIndex>, LabelIndex),
    Return,
    Call(FunctionIndex),
    CallIndirect(TableIndex, TypeUse),
}

pub struct BlockType {
    signature: TypeIndex,
    value_type: ValueType,
}

pub struct MemoryArgument {
    offset: usize,
    align: usize,
}

pub enum ConstantInstruction {}

pub enum StorageSize {
    I32_8,
    I64_8,
    I32_16,
    I64_16,
    I64_32,
}

pub enum SignExtension {
    Signed,
    Unsigned,
}
