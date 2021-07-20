use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::{Bytes, Name};
use byteorder::{LittleEndian, WriteBytesExt};
use std::borrow::Borrow;
use std::io::Write;
use std::mem::size_of;

/// See https://webassembly.github.io/spec/core/binary/values.html
impl Emit for i32 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        (*self as i64).emit(output)
    }
}

impl Emit for i64 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        Ok(leb128::write::signed(output, *self)?)
    }
}

impl Emit for u8 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        output.write_u8(*self)?;
        Ok(size_of::<u8>())
    }
}

impl Emit for u32 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        (*self as u64).emit(output)
    }
}

impl Emit for u64 {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        Ok(leb128::write::unsigned(output, *self)?)
    }
}

impl Emit for usize {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        (*self as u64).emit(output)
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
        self.as_bytes().emit(output)
    }
}

impl<'a> Emit for Bytes<'a> {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        for item in self.as_slice() {
            bytes += item.emit(output)?;
        }

        Ok(bytes)
    }
}

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

fn emit_byte<T: Borrow<u8>, O: Write>(byte: T, output: &mut O) -> Result<usize, CompilerError> {
    output.write_u8(*byte.borrow())?;
    Ok(size_of::<u8>())
}

fn emit_usize<T: Borrow<usize>, O: Write>(size: T, output: &mut O) -> Result<usize, CompilerError> {
    emit_u64(*size.borrow() as u64, output)
}

fn emit_u64<T: Borrow<u64>, O: Write>(size: T, output: &mut O) -> Result<usize, CompilerError> {
    Ok(leb128::write::unsigned(output, *size.borrow())?)
}

fn emit_vector<'items, I, E, O>(
    items: &'items [I],
    output: &mut O,
    emit: E,
) -> Result<usize, CompilerError>
where
    O: Write,
    E: Fn(&'items I, &mut O) -> Result<usize, CompilerError>,
{
    let mut bytes = 0;

    bytes += emit_usize(items.len(), output)?;
    bytes += emit_repeated(items, output, emit)?;

    Ok(bytes)
}

fn emit_repeated<'items, I, E, O>(
    items: &'items [I],
    output: &mut O,
    emit: E,
) -> Result<usize, CompilerError>
where
    O: Write,
    E: Fn(&'items I, &mut O) -> Result<usize, CompilerError>,
{
    let mut bytes = 0;

    for item in items {
        bytes += emit(item, output)?;
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectored() {
        let bytes: [u8; 4] = [1, 2, 3, 4];
        let mut buffer: Vec<u8> = Vec::new();

        let emitted = emit_vector(&bytes, &mut buffer, emit_byte).unwrap();

        assert_eq!(emitted, 1 + bytes.len());
        assert_eq!(buffer[0] as usize, bytes.len());
        assert_eq!(&bytes[..], &buffer[1..]);
    }
}
