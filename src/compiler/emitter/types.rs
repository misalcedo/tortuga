use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{
    FunctionType, GlobalType, Limit, MemoryType, NumberType, ReferenceType, ResultType, TableType,
    ValueType,
};
use byteorder::WriteBytesExt;
use std::io::Write;
use std::mem::size_of;

impl Emit for NumberType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let value: u8 = match self {
            NumberType::I32 => 0x7F,
            NumberType::I64 => 0x7E,
            NumberType::F32 => 0x7D,
            NumberType::F64 => 0x7C,
        };

        output.write_u8(value)?;

        Ok(size_of::<u8>())
    }
}

impl Emit for ReferenceType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let value: u8 = match self {
            ReferenceType::Function => 0x70,
            ReferenceType::External => 0x6F,
        };

        output.write_u8(value)?;

        Ok(size_of::<u8>())
    }
}

impl Emit for ValueType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        match self {
            ValueType::Number(number_type) => number_type.emit(output),
            ValueType::Reference(reference_type) => reference_type.emit(output),
        }
    }
}

impl Emit for ResultType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        self.value_types().emit(output)
    }
}

impl Emit for FunctionType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        output.write_u8(0x60)?;

        let mut bytes = size_of::<u8>();

        bytes += self.parameters().emit(output)?;
        bytes += self.results().emit(output)?;

        Ok(bytes)
    }
}

impl Emit for Limit {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = size_of::<u8>();

        match self.max() {
            Some(max) => {
                output.write_u8(0x00)?;

                bytes += self.min().emit(output)?;
                bytes += max.emit(output)?;
            }
            None => {
                output.write_u8(0x01)?;

                bytes += self.min().emit(output)?;
            }
        };

        Ok(bytes)
    }
}

impl Emit for MemoryType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        self.limits().emit(output)
    }
}

impl Emit for TableType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.reference_type().emit(output)?;
        bytes += self.limits().emit(output)?;

        Ok(bytes)
    }
}

impl Emit for GlobalType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.value_type().emit(output)?;

        let mutability: u8 = match self.is_mutable() {
            false => 0x00,
            true => 0x01,
        };

        output.write_u8(mutability)?;

        bytes += size_of::<u8>();

        Ok(bytes)
    }
}
