use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{Expression, Instruction};
use byteorder::WriteBytesExt;
use std::io::Write;
use std::mem::size_of;

impl Emit for Expression {
    fn emit<O: Write>(&self, mut output: O) -> Result<usize, CompilerError> {
        let mut bytes = size_of::<u8>();

        for instruction in self.instructions() {
            bytes += instruction.emit(&mut output)?;
        }

        output.write_u8(0x0B)?;

        Ok(bytes)
    }
}

impl Emit for Instruction {
    fn emit<O: Write>(&self, mut output: O) -> Result<usize, CompilerError> {
        let mut bytes = size_of::<u8>();

        match self {
            Instruction::ReferenceNull(kind) => {
                output.write_u8(0xD0)?;
                bytes += kind.emit(&mut output)?;
            }
            Instruction::ReferenceIsNull => {
                output.write_u8(0xD1)?;
            }
            Instruction::ReferenceFunction(index) => {
                output.write_u8(0xD2)?;
                bytes += index.emit(&mut output)?;
            }
            Instruction::Drop => {
                output.write_u8(0x1A)?;
            }
            Instruction::Select(types) => {
                if types.is_empty() {
                    output.write_u8(0x1B)?;
                } else {
                    output.write_u8(0x1C)?;
                    bytes += types.emit(&mut output)?;
                }
            }
        };

        Ok(bytes)
    }
}
