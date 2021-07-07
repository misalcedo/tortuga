use crate::compiler::errors::CompilerError;
use crate::web_assembly::{Limit, Module, Name, NumberType};
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;
use std::mem::size_of;

/// Emits a representation of an Abstract Syntax Tree (AST) to a `Write` output.
pub trait Emitter<T> {
    fn emit<O: Write>(&self, node: &T, output: O) -> Result<usize, CompilerError>;
}

pub struct BinaryWebAssemblyEmitter {}

impl BinaryWebAssemblyEmitter {
    pub fn new() -> BinaryWebAssemblyEmitter {
        BinaryWebAssemblyEmitter {}
    }
}

/// See https://webassembly.github.io/spec/core/binary/modules.html
impl Emitter<Module> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, module: &Module, output: O) -> Result<usize, CompilerError> {
        Ok(0)
    }
}

impl Emitter<NumberType> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(
        &self,
        number_type: &NumberType,
        mut output: O,
    ) -> Result<usize, CompilerError> {
        let value: u8 = match number_type {
            NumberType::I32 => 0x7F,
            NumberType::I64 => 0x7E,
            NumberType::F32 => 0x7D,
            NumberType::F64 => 0x7C,
        };

        output.write_u8(value)?;

        Ok(1)
    }
}

impl Emitter<Limit> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, limit: &Limit, mut output: O) -> Result<usize, CompilerError> {
        match limit {
            Limit::Min(min) => {
                output.write_u8(0x00)?;
                let mut bytes = 1;

                bytes += leb128::write::unsigned(&mut output, *min as u64)?;

                Ok(bytes)
            }
            Limit::MinMax { min, max } => {
                output.write_u8(0x01)?;

                let mut bytes = 1;

                bytes += self.emit(min, &mut output)?;
                bytes += self.emit(max, &mut output)?;

                Ok(bytes)
            }
        }
    }
}

/// See https://webassembly.github.io/spec/core/binary/values.html
impl Emitter<i32> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, number: &i32, mut output: O) -> Result<usize, CompilerError> {
        Ok(leb128::write::signed(&mut output, *number as i64)?)
    }
}

impl Emitter<i64> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, number: &i64, mut output: O) -> Result<usize, CompilerError> {
        Ok(leb128::write::signed(&mut output, *number)?)
    }
}

impl Emitter<u32> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, number: &u32, mut output: O) -> Result<usize, CompilerError> {
        Ok(leb128::write::unsigned(&mut output, *number as u64)?)
    }
}

impl Emitter<u64> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, number: &u64, mut output: O) -> Result<usize, CompilerError> {
        Ok(leb128::write::unsigned(&mut output, *number)?)
    }
}

impl Emitter<usize> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, number: &usize, mut output: O) -> Result<usize, CompilerError> {
        Ok(leb128::write::unsigned(&mut output, *number as u64)?)
    }
}

impl Emitter<f32> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, number: &f32, mut output: O) -> Result<usize, CompilerError> {
        output.write_f32::<LittleEndian>(*number)?;

        Ok(size_of::<f32>())
    }
}

impl Emitter<f64> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, number: &f64, mut output: O) -> Result<usize, CompilerError> {
        output.write_f64::<LittleEndian>(*number)?;

        Ok(size_of::<f64>())
    }
}

impl Emitter<Name> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, name: &Name, mut output: O) -> Result<usize, CompilerError> {
        Ok(output.write(name.as_bytes())?)
    }
}
