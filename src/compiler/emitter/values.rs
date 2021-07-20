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
        emit_bytes(self.as_bytes(), output, true)
    }
}

impl<'a> Emit for Bytes<'a> {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        for item in self.as_slice() {
            bytes += emit_byte(item, output)?;
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

        bytes += emit_usize(self.len(), output)?;

        for item in self {
            bytes += item.emit(output)?;
        }

        Ok(bytes)
    }
}

/// Emits a single byte to the output.
/// See https://webassembly.github.io/spec/core/binary/values.html#bytes
pub fn emit_byte<T: Borrow<u8>, O: Write>(byte: T, output: &mut O) -> Result<usize, CompilerError> {
    output.write_u8(*byte.borrow())?;
    Ok(size_of::<u8>())
}

/// Emits a slice of bytes to the output.
/// The bytes may optionally be treated as a vector.
/// Provides an optimization over using `emit_vector(value, output, emit_byte)` and `emit_repeated(value, output, emit_byte)`.
///
/// See https://webassembly.github.io/spec/core/binary/values.html#bytes
///
/// See https://webassembly.github.io/spec/core/binary/conventions.html#vectors
pub fn emit_bytes<O: Write>(
    value: &[u8],
    output: &mut O,
    include_length: bool,
) -> Result<usize, CompilerError> {
    let prefix = if include_length {
        emit_usize(value.len(), output)?
    } else {
        0
    };

    output.write_all(value)?;

    Ok(prefix + value.len())
}

/// Emits an unsigned 32-bit integer to the output.
/// See https://webassembly.github.io/spec/core/binary/values.html#integers
pub fn emit_u32<T: Borrow<u32>, O: Write>(
    value: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_u64(*value.borrow() as u64, output)
}

/// Emits an unsigned platform-specific (i.e., 32-bit or 64-bit) integer to the output.
/// See https://webassembly.github.io/spec/core/binary/values.html#integers
pub fn emit_usize<T: Borrow<usize>, O: Write>(
    size: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_u64(*size.borrow() as u64, output)
}

/// Emits an unsigned 64-bit integer to the output.
/// See https://webassembly.github.io/spec/core/binary/values.html#integers
pub fn emit_u64<T: Borrow<u64>, O: Write>(
    value: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    Ok(leb128::write::unsigned(output, *value.borrow())?)
}

/// Emits a signed 32-bit integer to the output.
/// See https://webassembly.github.io/spec/core/binary/values.html#integers
pub fn emit_i32<T: Borrow<i32>, O: Write>(
    value: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_i64(*value.borrow() as i64, output)
}

/// Emits a signed 64-bit integer to the output.
/// See https://webassembly.github.io/spec/core/binary/values.html#integers
pub fn emit_i64<T: Borrow<i64>, O: Write>(
    value: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    Ok(leb128::write::signed(output, *value.borrow())?)
}

/// Emit each item to the output using the given emit function.
/// Prefixes the items with the length of the slice.
/// See https://webassembly.github.io/spec/core/binary/conventions.html#vectors
pub fn emit_vector<'items, I, E, O>(
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

/// Emit each item to the output using the given emit function.
pub fn emit_repeated<'items, I, E, O>(
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
