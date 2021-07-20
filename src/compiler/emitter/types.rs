use crate::compiler::emitter::{emit_byte, emit_usize, emit_vector};
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::{
    FunctionType, GlobalType, Limit, MemoryType, NumberType, ReferenceType, ResultType, TableType,
    ValueType,
};
use std::borrow::Borrow;
use std::io::Write;

pub fn emit_number_type<T: Borrow<NumberType>, O: Write>(
    kind: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let value: u8 = match kind.borrow() {
        NumberType::I32 => 0x7F,
        NumberType::I64 => 0x7E,
        NumberType::F32 => 0x7D,
        NumberType::F64 => 0x7C,
    };

    emit_byte(value, output)
}

pub fn emit_reference_type<T: Borrow<ReferenceType>, O: Write>(
    kind: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let value: u8 = match kind.borrow() {
        ReferenceType::Function => 0x70,
        ReferenceType::External => 0x6F,
    };

    emit_byte(value, output)
}

pub fn emit_value_type<T: Borrow<ValueType>, O: Write>(
    kind: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    match kind.borrow() {
        ValueType::Number(number_type) => emit_number_type(number_type, output),
        ValueType::Reference(reference_type) => emit_reference_type(reference_type, output),
    }
}

pub fn emit_result_type<T: Borrow<ResultType>, O: Write>(
    kind: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_vector(kind.borrow().kinds(), output, emit_value_type)
}

pub fn emit_function_type<T: Borrow<FunctionType>, O: Write>(
    kind: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let kind = kind.borrow();

    bytes += emit_byte(0x60u8, output)?;
    bytes += emit_result_type(kind.parameters(), output)?;
    bytes += emit_result_type(kind.results(), output)?;

    Ok(bytes)
}

pub fn emit_limit<T: Borrow<Limit>, O: Write>(
    limits: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let limits = limits.borrow();

    match limits.max() {
        Some(max) => {
            bytes += emit_byte(0x01u8, output)?;
            bytes += emit_usize(limits.min(), output)?;
            bytes += emit_usize(max, output)?;
        }
        None => {
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_usize(limits.min(), output)?;
        }
    };

    Ok(bytes)
}

pub fn emit_memory_type<T: Borrow<MemoryType>, O: Write>(
    kind: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_limit(kind.borrow().limits(), output)
}

pub fn emit_table_type<T: Borrow<TableType>, O: Write>(
    kind: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let kind = kind.borrow();

    bytes += emit_reference_type(kind.kind(), output)?;
    bytes += emit_limit(kind.limits(), output)?;

    Ok(bytes)
}

pub fn emit_global_type<T: Borrow<GlobalType>, O: Write>(
    kind: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let kind = kind.borrow();

    bytes += emit_value_type(kind.kind(), output)?;

    let mutability: u8 = match kind.is_mutable() {
        false => 0x00,
        true => 0x01,
    };

    bytes += emit_byte(mutability, output)?;

    Ok(bytes)
}
