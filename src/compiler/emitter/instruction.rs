use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{
    BlockType, Expression, Instruction, MemoryArgument, NumberType, SignExtension, StorageSize,
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
        let mut bytes = 0;

        match self {
            Instruction::Unreachable => {
                bytes += 0x00u8.emit(output)?;
            }
            Instruction::Nop => {
                bytes += 0x01u8.emit(output)?;
            }
            Instruction::Block(kind, expression) => {
                bytes += 0x02u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Instruction::Loop(kind, expression) => {
                bytes += 0x03u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Instruction::If(kind, positive, negative) => {
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
            Instruction::Branch(index) => {
                bytes += 0x0Cu8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::BranchIf(index) => {
                bytes += 0x0Du8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::BranchTable(indices, index) => {
                bytes += 0x0Eu8.emit(output)?;
                bytes += indices.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::Return => {
                bytes += 0x0Fu8.emit(output)?;
            }
            Instruction::Call(index) => {
                bytes += 0x10u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::CallIndirect(table, kind) => {
                bytes += 0x11u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += table.emit(output)?;
            }
            Instruction::ReferenceNull(kind) => {
                bytes += 0xD0u8.emit(output)?;
                bytes += kind.emit(output)?;
            }
            Instruction::ReferenceIsNull => {
                bytes += 0xD1u8.emit(output)?;
            }
            Instruction::ReferenceFunction(index) => {
                bytes += 0xD2u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::Drop => {
                bytes += 0x1Au8.emit(output)?;
            }
            Instruction::Select(types) => {
                if types.is_empty() {
                    bytes += 0x1Bu8.emit(output)?;
                } else {
                    bytes += 0x1Cu8.emit(output)?;
                    bytes += types.emit(output)?;
                }
            }
            Instruction::LocalGet(index) => {
                bytes += 0x20u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::LocalSet(index) => {
                bytes += 0x21u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::LocalTee(index) => {
                bytes += 0x22u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::GlobalGet(index) => {
                bytes += 0x23u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::GlobalSet(index) => {
                bytes += 0x24u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableGet(index) => {
                bytes += 0x25u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableSet(index) => {
                bytes += 0x26u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableInit(element, table) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 12u32.emit(output)?;
                bytes += element.emit(output)?;
                bytes += table.emit(output)?;
            }
            Instruction::ElementDrop(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 13u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableCopy(table_a, table_b) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 14u32.emit(output)?;
                bytes += table_a.emit(output)?;
                bytes += table_b.emit(output)?;
            }
            Instruction::TableGrow(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 15u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableSize(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 16u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableFill(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 17u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::Load(NumberType::I32, memory_argument) => {
                bytes += 0x28u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::Load(NumberType::I64, memory_argument) => {
                bytes += 0x29u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::Load(NumberType::F32, memory_argument) => {
                bytes += 0x2Au8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::Load(NumberType::F64, memory_argument) => {
                bytes += 0x2Bu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I32_8,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += 0x2Cu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I32_8,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += 0x2Du8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I32_16,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += 0x2Eu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I32_16,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += 0x2Fu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I64_8,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += 0x30u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I64_8,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += 0x31u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I64_16,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += 0x32u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I64_16,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += 0x33u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I64_32,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += 0x34u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::LoadPartial(
                StorageSize::I64_32,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += 0x35u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::Store(NumberType::I32, memory_argument) => {
                bytes += 0x36u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::Store(NumberType::I64, memory_argument) => {
                bytes += 0x37u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::Store(NumberType::F32, memory_argument) => {
                bytes += 0x38u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::Store(NumberType::F64, memory_argument) => {
                bytes += 0x39u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::StorePartial(StorageSize::I32_8, memory_argument) => {
                bytes += 0x3Au8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::StorePartial(StorageSize::I32_16, memory_argument) => {
                bytes += 0x3Bu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::StorePartial(StorageSize::I64_8, memory_argument) => {
                bytes += 0x3Cu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::StorePartial(StorageSize::I64_16, memory_argument) => {
                bytes += 0x3Du8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::StorePartial(StorageSize::I64_32, memory_argument) => {
                bytes += 0x3Eu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Instruction::MemorySize => {
                bytes += 0x3Fu8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Instruction::MemoryGrow => {
                bytes += 0x40u8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Instruction::MemoryInit(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 8u32.emit(output)?;
                bytes += index.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Instruction::DatDrop(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 9u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::MemoryCopy => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 10u32.emit(output)?;
                bytes += 0x00u8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Instruction::MemoryFill => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 11u32.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Instruction::I32Constant(_) => {}
            Instruction::I64Constant(_) => {}
            Instruction::F32Constant(_) => {}
            Instruction::F64Constant(_) => {}
            Instruction::CountLeadingZeros(_) => {}
            Instruction::CountTrailingZeros(_) => {}
            Instruction::CountOnes(_) => {}
            Instruction::AbsoluteValue(_) => {}
            Instruction::Negate(_) => {}
            Instruction::SquareRoot(_) => {}
            Instruction::Ceiling(_) => {}
            Instruction::Floor(_) => {}
            Instruction::Truncate(_) => {}
            Instruction::Nearest(_) => {}
            Instruction::Add(_) => {}
            Instruction::Subtract(_) => {}
            Instruction::Multiply(_) => {}
            Instruction::DivideInteger(_, _) => {}
            Instruction::DivideFloat(_) => {}
            Instruction::Remainder(_, _) => {}
            Instruction::And(_) => {}
            Instruction::Or(_) => {}
            Instruction::Xor(_) => {}
            Instruction::ShiftLeft(_) => {}
            Instruction::ShiftRight(_, _) => {}
            Instruction::RotateLeft(_) => {}
            Instruction::RotateRight(_) => {}
            Instruction::Minimum(_) => {}
            Instruction::Maximum(_) => {}
            Instruction::CopySign(_) => {}
            Instruction::EqualToZero(_) => {}
            Instruction::Equal(_) => {}
            Instruction::NotEqual(_) => {}
            Instruction::LessThanInteger(_, _) => {}
            Instruction::LessThanFloat(_) => {}
            Instruction::GreaterThanInteger(_, _) => {}
            Instruction::GreaterThanFloat(_) => {}
            Instruction::LessThanOrEqualToInteger(_, _) => {}
            Instruction::LessThanOrEqualToFloat(_) => {}
            Instruction::GreaterThanOrEqualToInteger(_, _) => {}
            Instruction::GreaterThanOrEqualToFloat(_) => {}
            Instruction::Extend(_) => {}
            Instruction::Wrap => {}
            Instruction::ExtendWithSignExtension(_) => {}
            Instruction::ConvertAndTruncate(_, _, _) => {}
            Instruction::ConvertAndTruncateWithSaturation(_, _, _) => {}
            Instruction::Demote => {}
            Instruction::Promote => {}
            Instruction::Convert(_, _, _) => {}
            Instruction::ReinterpretFloat(_, _) => {}
            Instruction::ReinterpretInteger(_, _) => {}
        };

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
