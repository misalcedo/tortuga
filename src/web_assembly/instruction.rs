use crate::web_assembly::{
    DataIndex, ElementIndex, FloatType, FunctionIndex, GlobalIndex, IntegerType, LabelIndex,
    LocalIndex, NumberType, ReferenceType, TableIndex, TypeIndex, ValueType,
};

/// WebAssembly code consists of sequences of instructions.
/// Its computational model is based on a stack machine in that instructions manipulate values on
/// an implicit operand stack, consuming (popping) argument values and producing or returning
/// (pushing) result values.
/// In addition to dynamic operands from the stack,
/// some instructions also have static immediate arguments,
/// typically indices or type annotations, which are part of the instruction itself.
/// Some instructions are structured in that they bracket nested sequences of instructions.
/// The following sections group instructions into a number of different categories.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#instructions
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
    Block(BlockType, Expression),
    Loop(BlockType, Expression),
    If(BlockType, Expression, Option<Expression>),
    Unreachable,
    Nop,
    Branch(LabelIndex),
    BranchIf(LabelIndex),
    BranchTable(Vec<LabelIndex>, LabelIndex),
    Return,
    Call(FunctionIndex),
    CallIndirect(TypeIndex, TableIndex),
}

/// A structured instruction can consume input and produce output on the operand stack according to
/// its annotated block type.
/// It is given either as a type index that refers to a suitable function type,
/// or as an optional value type inline, which is a shorthand for the function type []→[valtype?].
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#control-instructions
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

/// Modifier to numeric operations (e.g.,  load, store, extend, etc.) to treat an integer as
/// smaller than its type suggest.
// TODO: get rid of this.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StorageSize {
    I32_8,
    I64_8,
    I32_16,
    I64_16,
    I64_32,
}

/// Some integer instructions come in two flavors, where a signedness annotation sx distinguishes
/// whether the operands are to be interpreted as unsigned or signed integers.
/// For the other integer instructions, the use of two’s complement for the signed interpretation
/// means that they behave the same regardless of signedness.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#numeric-instructions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SignExtension {
    Signed,
    Unsigned,
}

/// Function bodies, initialization values for globals,
/// and offsets of element or data segments are given as expressions, which are sequences of instructions terminated by an 𝖾𝗇𝖽 marker.
/// In some places, validation restricts expressions to be constant,
/// which limits the set of allowable instructions.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#expressions
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
