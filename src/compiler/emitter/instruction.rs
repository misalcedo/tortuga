use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{
    BlockType, ControlInstruction, Expression, Instruction, MemoryArgument, MemoryInstruction,
    NumberType, NumericInstruction, ParametricInstruction, ReferenceInstruction, SignExtension,
    StorageSize, TableInstruction, VariableInstruction,
};
use std::io::Write;

impl Emit for Expression {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        for instruction in self.instructions() {
            bytes += instruction.emit(output)?;
        }

        bytes += 0x0Bu8.emit(output)?;

        Ok(bytes)
    }
}

impl Emit for Instruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        match self {
            Self::Numeric(instruction) => instruction.emit(output),
            Self::Reference(instruction) => instruction.emit(output),
            Self::Parametric(instruction) => instruction.emit(output),
            Self::Variable(instruction) => instruction.emit(output),
            Self::Table(instruction) => instruction.emit(output),
            Self::Memory(instruction) => instruction.emit(output),
            Self::Control(instruction) => instruction.emit(output),
        }
    }
}

impl Emit for NumericInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::I32Constant(_) => {}
            Self::I64Constant(_) => {}
            Self::F32Constant(_) => {}
            Self::F64Constant(_) => {}
            Self::CountLeadingZeros(_) => {}
            Self::CountTrailingZeros(_) => {}
            Self::CountOnes(_) => {}
            Self::AbsoluteValue(_) => {}
            Self::Negate(_) => {}
            Self::SquareRoot(_) => {}
            Self::Ceiling(_) => {}
            Self::Floor(_) => {}
            Self::Truncate(_) => {}
            Self::Nearest(_) => {}
            Self::Add(_) => {}
            Self::Subtract(_) => {}
            Self::Multiply(_) => {}
            Self::DivideInteger(_, _) => {}
            Self::DivideFloat(_) => {}
            Self::Remainder(_, _) => {}
            Self::And(_) => {}
            Self::Or(_) => {}
            Self::Xor(_) => {}
            Self::ShiftLeft(_) => {}
            Self::ShiftRight(_, _) => {}
            Self::RotateLeft(_) => {}
            Self::RotateRight(_) => {}
            Self::Minimum(_) => {}
            Self::Maximum(_) => {}
            Self::CopySign(_) => {}
            Self::EqualToZero(_) => {}
            Self::Equal(_) => {}
            Self::NotEqual(_) => {}
            Self::LessThanInteger(_, _) => {}
            Self::LessThanFloat(_) => {}
            Self::GreaterThanInteger(_, _) => {}
            Self::GreaterThanFloat(_) => {}
            Self::LessThanOrEqualToInteger(_, _) => {}
            Self::LessThanOrEqualToFloat(_) => {}
            Self::GreaterThanOrEqualToInteger(_, _) => {}
            Self::GreaterThanOrEqualToFloat(_) => {}
            Self::Extend(_) => {}
            Self::Wrap => {}
            Self::ExtendWithSignExtension(_) => {}
            Self::ConvertAndTruncate(_, _, _) => {}
            Self::ConvertAndTruncateWithSaturation(_, _, _) => {}
            Self::Demote => {}
            Self::Promote => {}
            Self::Convert(_, _, _) => {}
            Self::ReinterpretFloat(_, _) => {}
            Self::ReinterpretInteger(_, _) => {}
        }

        Ok(bytes)
    }
}

impl Emit for ReferenceInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::ReferenceNull(kind) => {
                bytes += 0xD0u8.emit(output)?;
                bytes += kind.emit(output)?;
            }
            Self::ReferenceIsNull => {
                bytes += 0xD1u8.emit(output)?;
            }
            Self::ReferenceFunction(index) => {
                bytes += 0xD2u8.emit(output)?;
                bytes += index.emit(output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for ParametricInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::Drop => {
                bytes += 0x1Au8.emit(output)?;
            }
            Self::Select(Some(types)) => {
                bytes += 0x1Cu8.emit(output)?;
                bytes += types.emit(output)?;
            }
            Self::Select(None) => {
                bytes += 0x1Bu8.emit(output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for VariableInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::LocalGet(index) => {
                bytes += 0x20u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::LocalSet(index) => {
                bytes += 0x21u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::LocalTee(index) => {
                bytes += 0x22u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::GlobalGet(index) => {
                bytes += 0x23u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::GlobalSet(index) => {
                bytes += 0x24u8.emit(output)?;
                bytes += index.emit(output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for TableInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::TableGet(index) => {
                bytes += 0x25u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableSet(index) => {
                bytes += 0x26u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableInit(element, table) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 12u32.emit(output)?;
                bytes += element.emit(output)?;
                bytes += table.emit(output)?;
            }
            Self::ElementDrop(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 13u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableCopy(table_a, table_b) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 14u32.emit(output)?;
                bytes += table_a.emit(output)?;
                bytes += table_b.emit(output)?;
            }
            Self::TableGrow(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 15u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableSize(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 16u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableFill(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 17u32.emit(output)?;
                bytes += index.emit(output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for MemoryInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::Load(NumberType::I32, memory_argument) => {
                bytes += 0x28u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::I64, memory_argument) => {
                bytes += 0x29u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::F32, memory_argument) => {
                bytes += 0x2Au8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::F64, memory_argument) => {
                bytes += 0x2Bu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_8, SignExtension::Signed, memory_argument) => {
                bytes += 0x2Cu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_8, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x2Du8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_16, SignExtension::Signed, memory_argument) => {
                bytes += 0x2Eu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_16, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x2Fu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_8, SignExtension::Signed, memory_argument) => {
                bytes += 0x30u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_8, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x31u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_16, SignExtension::Signed, memory_argument) => {
                bytes += 0x32u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_16, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x33u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_32, SignExtension::Signed, memory_argument) => {
                bytes += 0x34u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_32, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x35u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::I32, memory_argument) => {
                bytes += 0x36u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::I64, memory_argument) => {
                bytes += 0x37u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::F32, memory_argument) => {
                bytes += 0x38u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::F64, memory_argument) => {
                bytes += 0x39u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I32_8, memory_argument) => {
                bytes += 0x3Au8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I32_16, memory_argument) => {
                bytes += 0x3Bu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_8, memory_argument) => {
                bytes += 0x3Cu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_16, memory_argument) => {
                bytes += 0x3Du8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_32, memory_argument) => {
                bytes += 0x3Eu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::MemorySize => {
                bytes += 0x3Fu8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Self::MemoryGrow => {
                bytes += 0x40u8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Self::MemoryInit(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 8u32.emit(output)?;
                bytes += index.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Self::DataDrop(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 9u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::MemoryCopy => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 10u32.emit(output)?;
                bytes += 0x00u8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Self::MemoryFill => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 11u32.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for ControlInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::Unreachable => {
                bytes += 0x00u8.emit(output)?;
            }
            Self::Nop => {
                bytes += 0x01u8.emit(output)?;
            }
            Self::Block(kind, expression) => {
                bytes += 0x02u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Self::Loop(kind, expression) => {
                bytes += 0x03u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Self::If(kind, positive, negative) => {
                bytes += 0x04u8.emit(output)?;
                bytes += kind.emit(output)?;

                if let Some(negative) = negative {
                    for instruction in positive.instructions() {
                        bytes += instruction.emit(output)?;
                    }

                    bytes += 0x05u8.emit(output)?;
                    bytes += negative.emit(output)?;
                } else {
                    bytes += positive.emit(output)?;
                }
            }
            Self::Branch(index) => {
                bytes += 0x0Cu8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::BranchIf(index) => {
                bytes += 0x0Du8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::BranchTable(indices, index) => {
                bytes += 0x0Eu8.emit(output)?;
                bytes += indices.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::Return => {
                bytes += 0x0Fu8.emit(output)?;
            }
            Self::Call(index) => {
                bytes += 0x10u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::CallIndirect(table, kind) => {
                bytes += 0x11u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += table.emit(output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for BlockType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        match self {
            BlockType::Index(index) => (*index as i64).emit(output),
            BlockType::ValueType(kind) => kind.emit(output),
            BlockType::None => 0x40u8.emit(output),
        }
    }
}

impl Emit for MemoryArgument {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.align().emit(output)?;
        bytes += self.offset().emit(output)?;

        Ok(bytes)
    }
}
