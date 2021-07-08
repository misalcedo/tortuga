use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::Name;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;
use std::mem::size_of;

/// See https://webassembly.github.io/spec/core/binary/values.html
impl Emit for i32 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        Ok(leb128::write::signed(output, *self as i64)?)
    }
}

impl Emit for i64 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        Ok(leb128::write::signed(output, *self)?)
    }
}

impl Emit for u32 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        Ok(leb128::write::unsigned(output, *self as u64)?)
    }
}

impl Emit for u64 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        Ok(leb128::write::unsigned(output, *self)?)
    }
}

impl Emit for usize {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        Ok(leb128::write::unsigned(output, *self as u64)?)
    }
}

impl Emit for f32 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        output.write_f32::<LittleEndian>(*self)?;

        Ok(size_of::<f32>())
    }
}

impl Emit for f64 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        output.write_f64::<LittleEndian>(*self)?;

        Ok(size_of::<f64>())
    }
}

impl Emit for Name {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.len().emit(output)?;
        bytes += self.as_bytes().emit(output)?;

        Ok(bytes)
    }
}

impl Emit for [u8] {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        Ok(output.write(self)?)
    }
}

// TODO create distinction between emit vector vs. emit so I can emit a u8 and write all of the size_of and write_u8 uses.
impl<T> Emit for [T]
where
    T: Emit,
{
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.len().emit(output)?;

        for item in self {
            bytes += item.emit(output)?;
        }

        Ok(bytes)
    }
}
