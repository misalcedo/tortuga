use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{BlockType, Expression, Instruction, MemoryArgument};
use byteorder::WriteBytesExt;
use std::io::Write;
use std::mem::size_of;

impl Emit for Expression {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = size_of::<u8>();

        for instruction in self.instructions() {
            bytes += instruction.emit(output)?;
        }

        output.write_u8(0x0B)?;

        Ok(bytes)
    }
}

impl Emit for Instruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = size_of::<u8>();

        match self {
            Instruction::Unreachable => {
                output.write_u8(0x00)?;
            }
            Instruction::Nop => {
                output.write_u8(0x01)?;
            }
            Instruction::Block { expression, kind } => {
                output.write_u8(0x02)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Instruction::Loop { expression, kind } => {
                output.write_u8(0x03)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Instruction::If {
                positive,
                negative,
                kind,
            } => {
                output.write_u8(0x04)?;

                bytes += kind.emit(output)?;

                if let Some(negative) = negative {
                    for instruction in positive.instructions() {
                        bytes += instruction.emit(output)?;
                    }

                    output.write_u8(0x05)?;

                    bytes += size_of::<u8>();
                    bytes += negative.emit(output)?;
                } else {
                    bytes += positive.emit(output)?;
                }
            }
            Instruction::Branch(index) => {
                output.write_u8(0x0C)?;
                bytes += index.emit(output)?;
            }
            Instruction::BranchIf(index) => {
                output.write_u8(0x0D)?;
                bytes += index.emit(output)?;
            }
            Instruction::BranchTable(indices, index) => {
                output.write_u8(0x0E)?;
                bytes += indices.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::Return => {
                output.write_u8(0x0F)?;
            }
            Instruction::Call(index) => {
                output.write_u8(0x10)?;
                bytes += index.emit(output)?;
            }
            Instruction::CallIndirect(table, kind) => {
                output.write_u8(0x11)?;
                bytes += kind.emit(output)?;
                bytes += table.emit(output)?;
            }
            Instruction::ReferenceNull(kind) => {
                output.write_u8(0xD0)?;
                bytes += kind.emit(output)?;
            }
            Instruction::ReferenceIsNull => {
                output.write_u8(0xD1)?;
            }
            Instruction::ReferenceFunction(index) => {
                output.write_u8(0xD2)?;
                bytes += index.emit(output)?;
            }
            Instruction::Drop => {
                output.write_u8(0x1A)?;
            }
            Instruction::Select(types) => {
                if types.is_empty() {
                    output.write_u8(0x1B)?;
                } else {
                    output.write_u8(0x1C)?;
                    bytes += types.emit(output)?;
                }
            }
            Instruction::LocalGet(index) => {
                output.write_u8(0x20)?;
                bytes += index.emit(output)?;
            }
            Instruction::LocalSet(index) => {
                output.write_u8(0x21)?;
                bytes += index.emit(output)?;
            }
            Instruction::LocalTee(index) => {
                output.write_u8(0x22)?;
                bytes += index.emit(output)?;
            }
            Instruction::GlobalGet(index) => {
                output.write_u8(0x23)?;
                bytes += index.emit(output)?;
            }
            Instruction::GlobalSet(index) => {
                output.write_u8(0x24)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableGet(index) => {
                output.write_u8(0x25)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableSet(index) => {
                output.write_u8(0x26)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableInit(element, table) => {
                output.write_u8(0xFC)?;
                bytes += 12u32.emit(output)?;
                bytes += element.emit(output)?;
                bytes += table.emit(output)?;
            }
            Instruction::ElementDrop(index) => {
                output.write_u8(0xFC)?;
                bytes += 13u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableCopy(table_a, table_b) => {
                output.write_u8(0xFC)?;
                bytes += 14u32.emit(output)?;
                bytes += table_a.emit(output)?;
                bytes += table_b.emit(output)?;
            }
            Instruction::TableGrow(index) => {
                output.write_u8(0xFC)?;
                bytes += 15u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableSize(index) => {
                output.write_u8(0xFC)?;
                bytes += 16u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Instruction::TableFill(index) => {
                output.write_u8(0xFC)?;
                bytes += 17u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            _ => (),
        };

        Ok(bytes)
    }
}

impl Emit for BlockType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            BlockType::Index(index) => {
                bytes += (*index as i64).emit(output)?;
            }
            BlockType::ValueType(kind) => {
                bytes += kind.emit(output)?;
            }
            BlockType::None => {
                output.write_u8(0x40)?;
                bytes += size_of::<u8>();
            }
        }

        Ok(bytes)
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
