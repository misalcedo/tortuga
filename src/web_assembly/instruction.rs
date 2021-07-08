use crate::web_assembly::{
    DataIndex, ElementIndex, FloatType, FunctionIndex, GlobalIndex, IntegerType, LabelIndex,
    LocalIndex, NumberType, ReferenceType, TableIndex, TypeIndex, ValueType,
};

// TODO Update to have explicit instructions for every type pair, storage width, and sign extension to make emitting opcodes easier
/// Instructions are syntactically distinguished into plain and structured instructions.
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    // Numeric
    I32Constant(i32),
    I64Constant(i64),
    F32Constant(f32),
    F64Constant(f64),
    CountLeadingZeros(IntegerType),  // clz
    CountTrailingZeros(IntegerType), // ctz
    CountOnes(IntegerType),          // popcnt
    AbsoluteValue(FloatType),
    Negate(FloatType),
    SquareRoot(FloatType),
    Ceiling(FloatType),
    Floor(FloatType),
    Truncate(FloatType),
    Nearest(FloatType),
    Add(NumberType),
    Subtract(NumberType),
    Multiply(NumberType),
    DivideInteger(IntegerType, SignExtension),
    DivideFloat(FloatType),
    Remainder(IntegerType, SignExtension),
    And(IntegerType),
    Or(IntegerType),
    Xor(IntegerType),
    ShiftLeft(IntegerType),
    ShiftRight(IntegerType, SignExtension),
    RotateLeft(IntegerType),
    RotateRight(IntegerType),
    Minimum(FloatType),
    Maximum(FloatType),
    CopySign(FloatType),
    EqualToZero(IntegerType),
    Equal(NumberType),
    NotEqual(NumberType),
    LessThanInteger(IntegerType, SignExtension),
    LessThanFloat(FloatType),
    GreaterThanInteger(IntegerType, SignExtension),
    GreaterThanFloat(FloatType),
    LessThanOrEqualToInteger(IntegerType, SignExtension),
    LessThanOrEqualToFloat(FloatType),
    GreaterThanOrEqualToInteger(IntegerType, SignExtension),
    GreaterThanOrEqualToFloat(FloatType),
    Extend(StorageSize),
    Wrap,
    ExtendWithSignExtension(SignExtension),
    ConvertAndTruncate(IntegerType, FloatType, SignExtension), // trunc
    ConvertAndTruncateWithSaturation(IntegerType, FloatType, SignExtension), // trunc_sat
    Demote,
    Promote,
    Convert(FloatType, IntegerType, SignExtension),
    ReinterpretFloat(IntegerType, FloatType),
    ReinterpretInteger(FloatType, IntegerType),
    // Reference
    ReferenceNull(ReferenceType),
    ReferenceIsNull,
    ReferenceFunction(FunctionIndex),
    // Parametric
    Drop,
    Select(Vec<ValueType>),
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
    TableInit(ElementIndex, TableIndex),
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
        expression: Expression,
        kind: BlockType,
    },
    Loop {
        expression: Expression,
        kind: BlockType,
    },
    If {
        positive: Expression,
        negative: Option<Expression>,
        kind: BlockType,
    },
    Unreachable,
    Nop,
    Branch(LabelIndex),
    BranchIf(LabelIndex),
    BranchTable(Vec<LabelIndex>, LabelIndex),
    Return,
    Call(FunctionIndex),
    CallIndirect(TypeIndex, TableIndex),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockType {
    None,
    Index(TypeIndex),
    ValueType(ValueType),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MemoryArgument {
    align: usize,
    offset: usize,
}

impl MemoryArgument {
    pub fn new(align: usize, offset: usize) -> Self {
        MemoryArgument { align, offset }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn align(&self) -> usize {
        self.align
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StorageSize {
    I32_8,
    I64_8,
    I32_16,
    I64_16,
    I64_32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SignExtension {
    Signed,
    Unsigned,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    instructions: Vec<Instruction>,
}

impl Expression {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Expression { instructions }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_expression() {
        let instruction = Instruction::Nop;
        let expression = Expression::new(vec![instruction.clone()]);

        assert_eq!(expression.instructions(), &[instruction]);
        assert!(!expression.is_empty());
    }

    #[test]
    fn new_memory_argument() {
        let align = 0;
        let offset = 42;
        let argument = MemoryArgument::new(align, offset);

        assert_eq!(argument.align(), align);
        assert_eq!(argument.offset(), offset);
    }
}
