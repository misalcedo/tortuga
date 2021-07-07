use crate::compiler::emitter::{BinaryWebAssemblyEmitter, Emitter};
use crate::compiler::errors::CompilerError;
use crate::web_assembly::Name;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;
use std::mem::size_of;

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

impl Emitter<u8> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, number: &u8, mut output: O) -> Result<usize, CompilerError> {
        output.write_u8(*number)?;
        Ok(size_of::<u8>())
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
