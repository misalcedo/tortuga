use crate::compiler::emitter::values::*;
use crate::compiler::emitter::Emit;
use crate::compiler::CompilerError;
use crate::syntax::web_assembly::{
    BlockType, ControlInstruction, Expression, FloatType, Instruction, IntegerType, MemoryArgument,
    MemoryInstruction, NumberType, NumericInstruction, ParametricInstruction, ReferenceInstruction,
    SignExtension, StorageSize, TableInstruction, VariableInstruction,
};
use std::io::Write;

impl Emit for Expression {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        for instruction in self.instructions() {
            bytes += instruction.emit(output)?;
        }

        bytes += emit_byte(0x0Bu8, output)?;

        Ok(bytes)
    }
}

impl Emit for Instruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        match self {
            Self::Numeric(instruction) => instruction.emit(output),
            Self::Reference(instruction) => instruction.emit(output),
            Self::Parametric(instruction) => instruction.emit(output),
            Self::Variable(instruction) => instruction.emit(output),
            Self::Table(instruction) => instruction.emit(output),
            Self::Memory(instruction) => instruction.emit(output),
            Self::Control(instruction) => instruction.emit(output),
        }
    }
}

impl Emit for NumericInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            // Constant Operations
            Self::I32Constant(value) => {
                bytes += emit_byte(0x41u8, output)?;
                bytes += value.emit(output)?;
            }
            Self::I64Constant(value) => {
                bytes += emit_byte(0x42u8, output)?;
                bytes += value.emit(output)?;
            }
            Self::F32Constant(value) => {
                bytes += emit_byte(0x43u8, output)?;
                bytes += value.emit(output)?;
            }
            Self::F64Constant(value) => {
                bytes += emit_byte(0x44u8, output)?;
                bytes += value.emit(output)?;
            }
            // i32 Test Operations
            Self::EqualToZero(IntegerType::I32) => {
                bytes += emit_byte(0x45u8, output)?;
            }
            // i32 Relation Operations
            Self::Equal(NumberType::I32) => {
                bytes += emit_byte(0x46u8, output)?;
            }
            Self::NotEqual(NumberType::I32) => {
                bytes += emit_byte(0x47u8, output)?;
            }
            Self::LessThanInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0x48u8, output)?;
            }
            Self::LessThanInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0x49u8, output)?;
            }
            Self::GreaterThanInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0x4Au8, output)?;
            }
            Self::GreaterThanInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0x4Bu8, output)?;
            }
            Self::LessThanOrEqualToInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0x4Cu8, output)?;
            }
            Self::LessThanOrEqualToInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0x4Du8, output)?;
            }
            Self::GreaterThanOrEqualToInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0x4Eu8, output)?;
            }
            Self::GreaterThanOrEqualToInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0x4Fu8, output)?;
            }
            // i64 Test Operations
            Self::EqualToZero(IntegerType::I64) => {
                bytes += emit_byte(0x50u8, output)?;
            }
            // i64 Relation Operations
            Self::Equal(NumberType::I64) => {
                bytes += emit_byte(0x51u8, output)?;
            }
            Self::NotEqual(NumberType::I64) => {
                bytes += emit_byte(0x52u8, output)?;
            }
            Self::LessThanInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0x53u8, output)?;
            }
            Self::LessThanInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0x54u8, output)?;
            }
            Self::GreaterThanInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0x55u8, output)?;
            }
            Self::GreaterThanInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0x56u8, output)?;
            }
            Self::LessThanOrEqualToInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0x57u8, output)?;
            }
            Self::LessThanOrEqualToInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0x58u8, output)?;
            }
            Self::GreaterThanOrEqualToInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0x59u8, output)?;
            }
            Self::GreaterThanOrEqualToInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0x5Au8, output)?;
            }
            // f32 Relation Operations
            Self::Equal(NumberType::F32) => {
                bytes += emit_byte(0x5Bu8, output)?;
            }
            Self::NotEqual(NumberType::F32) => {
                bytes += emit_byte(0x5Cu8, output)?;
            }
            Self::LessThanFloat(FloatType::F32) => {
                bytes += emit_byte(0x5Du8, output)?;
            }
            Self::GreaterThanFloat(FloatType::F32) => {
                bytes += emit_byte(0x5Eu8, output)?;
            }
            Self::LessThanOrEqualToFloat(FloatType::F32) => {
                bytes += emit_byte(0x5Fu8, output)?;
            }
            Self::GreaterThanOrEqualToFloat(FloatType::F32) => {
                bytes += emit_byte(0x60u8, output)?;
            }
            // f64 Relation Operations
            Self::Equal(NumberType::F64) => {
                bytes += emit_byte(0x61u8, output)?;
            }
            Self::NotEqual(NumberType::F64) => {
                bytes += emit_byte(0x62u8, output)?;
            }
            Self::LessThanFloat(FloatType::F64) => {
                bytes += emit_byte(0x63u8, output)?;
            }
            Self::GreaterThanFloat(FloatType::F64) => {
                bytes += emit_byte(0x64u8, output)?;
            }
            Self::LessThanOrEqualToFloat(FloatType::F64) => {
                bytes += emit_byte(0x65u8, output)?;
            }
            Self::GreaterThanOrEqualToFloat(FloatType::F64) => {
                bytes += emit_byte(0x66u8, output)?;
            }
            // i32 Unary Operations
            Self::CountLeadingZeros(IntegerType::I32) => {
                bytes += emit_byte(0x67u8, output)?;
            }
            Self::CountTrailingZeros(IntegerType::I32) => {
                bytes += emit_byte(0x68u8, output)?;
            }
            Self::CountOnes(IntegerType::I32) => {
                bytes += emit_byte(0x69u8, output)?;
            }
            // i32 Binary Operations
            Self::Add(NumberType::I32) => {
                bytes += emit_byte(0x6Au8, output)?;
            }
            Self::Subtract(NumberType::I32) => {
                bytes += emit_byte(0x6Bu8, output)?;
            }
            Self::Multiply(NumberType::I32) => {
                bytes += emit_byte(0x6Cu8, output)?;
            }
            Self::DivideInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0x6Du8, output)?;
            }
            Self::DivideInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0x6Eu8, output)?;
            }
            Self::Remainder(IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0x6Fu8, output)?;
            }
            Self::Remainder(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0x70u8, output)?;
            }
            Self::And(IntegerType::I32) => {
                bytes += emit_byte(0x71u8, output)?;
            }
            Self::Or(IntegerType::I32) => {
                bytes += emit_byte(0x72u8, output)?;
            }
            Self::Xor(IntegerType::I32) => {
                bytes += emit_byte(0x73u8, output)?;
            }
            Self::ShiftLeft(IntegerType::I32) => {
                bytes += emit_byte(0x74u8, output)?;
            }
            Self::ShiftRight(IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0x75u8, output)?;
            }
            Self::ShiftRight(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0x76u8, output)?;
            }
            Self::RotateLeft(IntegerType::I32) => {
                bytes += emit_byte(0x77u8, output)?;
            }
            Self::RotateRight(IntegerType::I32) => {
                bytes += emit_byte(0x78u8, output)?;
            }
            // i64 Unary Operations
            Self::CountLeadingZeros(IntegerType::I64) => {
                bytes += emit_byte(0x79u8, output)?;
            }
            Self::CountTrailingZeros(IntegerType::I64) => {
                bytes += emit_byte(0x7Au8, output)?;
            }
            Self::CountOnes(IntegerType::I64) => {
                bytes += emit_byte(0x7Bu8, output)?;
            }
            // i64 Binary Operations
            Self::Add(NumberType::I64) => {
                bytes += emit_byte(0x7Cu8, output)?;
            }
            Self::Subtract(NumberType::I64) => {
                bytes += emit_byte(0x7Du8, output)?;
            }
            Self::Multiply(NumberType::I64) => {
                bytes += emit_byte(0x7Eu8, output)?;
            }
            Self::DivideInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0x7Fu8, output)?;
            }
            Self::DivideInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0x80u8, output)?;
            }
            Self::Remainder(IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0x81u8, output)?;
            }
            Self::Remainder(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0x82u8, output)?;
            }
            Self::And(IntegerType::I64) => {
                bytes += emit_byte(0x83u8, output)?;
            }
            Self::Or(IntegerType::I64) => {
                bytes += emit_byte(0x84u8, output)?;
            }
            Self::Xor(IntegerType::I64) => {
                bytes += emit_byte(0x85u8, output)?;
            }
            Self::ShiftLeft(IntegerType::I64) => {
                bytes += emit_byte(0x86u8, output)?;
            }
            Self::ShiftRight(IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0x87u8, output)?;
            }
            Self::ShiftRight(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0x88u8, output)?;
            }
            Self::RotateLeft(IntegerType::I64) => {
                bytes += emit_byte(0x89u8, output)?;
            }
            Self::RotateRight(IntegerType::I64) => {
                bytes += emit_byte(0x8Au8, output)?;
            }
            // f32 Unary Operations
            Self::AbsoluteValue(FloatType::F32) => {
                bytes += emit_byte(0x8Bu8, output)?;
            }
            Self::Negate(FloatType::F32) => {
                bytes += emit_byte(0x8Cu8, output)?;
            }
            Self::Ceiling(FloatType::F32) => {
                bytes += emit_byte(0x8Du8, output)?;
            }
            Self::Floor(FloatType::F32) => {
                bytes += emit_byte(0x8Eu8, output)?;
            }
            Self::Truncate(FloatType::F32) => {
                bytes += emit_byte(0x8Fu8, output)?;
            }
            Self::Nearest(FloatType::F32) => {
                bytes += emit_byte(0x90u8, output)?;
            }
            Self::SquareRoot(FloatType::F32) => {
                bytes += emit_byte(0x91u8, output)?;
            }
            // f32 Binary Operations
            Self::Add(NumberType::F32) => {
                bytes += emit_byte(0x92u8, output)?;
            }
            Self::Subtract(NumberType::F32) => {
                bytes += emit_byte(0x93u8, output)?;
            }
            Self::Multiply(NumberType::F32) => {
                bytes += emit_byte(0x94u8, output)?;
            }
            Self::DivideFloat(FloatType::F32) => {
                bytes += emit_byte(0x95u8, output)?;
            }
            Self::Minimum(FloatType::F32) => {
                bytes += emit_byte(0x96u8, output)?;
            }
            Self::Maximum(FloatType::F32) => {
                bytes += emit_byte(0x97u8, output)?;
            }
            Self::CopySign(FloatType::F32) => {
                bytes += emit_byte(0x98u8, output)?;
            }
            // f64 Unary Operations
            Self::AbsoluteValue(FloatType::F64) => {
                bytes += emit_byte(0x99u8, output)?;
            }
            Self::Negate(FloatType::F64) => {
                bytes += emit_byte(0x9Au8, output)?;
            }
            Self::Ceiling(FloatType::F64) => {
                bytes += emit_byte(0x9Bu8, output)?;
            }
            Self::Floor(FloatType::F64) => {
                bytes += emit_byte(0x9Cu8, output)?;
            }
            Self::Truncate(FloatType::F64) => {
                bytes += emit_byte(0x9Du8, output)?;
            }
            Self::Nearest(FloatType::F64) => {
                bytes += emit_byte(0x9Eu8, output)?;
            }
            Self::SquareRoot(FloatType::F64) => {
                bytes += emit_byte(0x9Fu8, output)?;
            }
            // f64 Binary Operations
            Self::Add(NumberType::F64) => {
                bytes += emit_byte(0xA0u8, output)?;
            }
            Self::Subtract(NumberType::F64) => {
                bytes += emit_byte(0xA1u8, output)?;
            }
            Self::Multiply(NumberType::F64) => {
                bytes += emit_byte(0xA2u8, output)?;
            }
            Self::DivideFloat(FloatType::F64) => {
                bytes += emit_byte(0xA3u8, output)?;
            }
            Self::Minimum(FloatType::F64) => {
                bytes += emit_byte(0xA4u8, output)?;
            }
            Self::Maximum(FloatType::F64) => {
                bytes += emit_byte(0xA5u8, output)?;
            }
            Self::CopySign(FloatType::F64) => {
                bytes += emit_byte(0xA6u8, output)?;
            }
            // Convert Operations
            Self::Wrap => {
                bytes += emit_byte(0xA7u8, output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I32, FloatType::F32, SignExtension::Signed) => {
                bytes += emit_byte(0xA8u8, output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I32, FloatType::F32, SignExtension::Unsigned) => {
                bytes += emit_byte(0xA9u8, output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I32, FloatType::F64, SignExtension::Signed) => {
                bytes += emit_byte(0xAAu8, output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I32, FloatType::F64, SignExtension::Unsigned) => {
                bytes += emit_byte(0xABu8, output)?;
            }
            Self::ExtendWithSignExtension(SignExtension::Signed) => {
                bytes += emit_byte(0xACu8, output)?;
            }
            Self::ExtendWithSignExtension(SignExtension::Unsigned) => {
                bytes += emit_byte(0xADu8, output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I64, FloatType::F32, SignExtension::Signed) => {
                bytes += emit_byte(0xAEu8, output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I64, FloatType::F32, SignExtension::Unsigned) => {
                bytes += emit_byte(0xAFu8, output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I64, FloatType::F64, SignExtension::Signed) => {
                bytes += emit_byte(0xB0u8, output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I64, FloatType::F64, SignExtension::Unsigned) => {
                bytes += emit_byte(0xB1u8, output)?;
            }
            Self::Convert(FloatType::F32, IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0xB2u8, output)?;
            }
            Self::Convert(FloatType::F32, IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0xB3u8, output)?;
            }
            Self::Convert(FloatType::F32, IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0xB4u8, output)?;
            }
            Self::Convert(FloatType::F32, IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0xB5u8, output)?;
            }
            Self::Demote => {
                bytes += emit_byte(0xB6u8, output)?;
            }
            Self::Convert(FloatType::F64, IntegerType::I32, SignExtension::Signed) => {
                bytes += emit_byte(0xB7u8, output)?;
            }
            Self::Convert(FloatType::F64, IntegerType::I32, SignExtension::Unsigned) => {
                bytes += emit_byte(0xB8u8, output)?;
            }
            Self::Convert(FloatType::F64, IntegerType::I64, SignExtension::Signed) => {
                bytes += emit_byte(0xB9u8, output)?;
            }
            Self::Convert(FloatType::F64, IntegerType::I64, SignExtension::Unsigned) => {
                bytes += emit_byte(0xBAu8, output)?;
            }
            Self::Promote => {
                bytes += emit_byte(0xBBu8, output)?;
            }
            Self::ReinterpretFloat(IntegerType::I32, FloatType::F32) => {
                bytes += emit_byte(0xBCu8, output)?;
            }
            Self::ReinterpretFloat(IntegerType::I64, FloatType::F64) => {
                bytes += emit_byte(0xBDu8, output)?;
            }
            Self::ReinterpretInteger(FloatType::F32, IntegerType::I32) => {
                bytes += emit_byte(0xBEu8, output)?;
            }
            Self::ReinterpretInteger(FloatType::F64, IntegerType::I64) => {
                bytes += emit_byte(0xBFu8, output)?;
            }
            Self::ExtendSigned(StorageSize::I32_8) => {
                bytes += emit_byte(0xC0u8, output)?;
            }
            Self::ExtendSigned(StorageSize::I32_16) => {
                bytes += emit_byte(0xC1u8, output)?;
            }
            Self::ExtendSigned(StorageSize::I64_8) => {
                bytes += emit_byte(0xC2u8, output)?;
            }
            Self::ExtendSigned(StorageSize::I64_16) => {
                bytes += emit_byte(0xC3u8, output)?;
            }
            Self::ExtendSigned(StorageSize::I64_32) => {
                bytes += emit_byte(0xC4u8, output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F32,
                SignExtension::Signed,
            ) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(0u32, output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F32,
                SignExtension::Unsigned,
            ) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(1u32, output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F64,
                SignExtension::Signed,
            ) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(2u32, output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F64,
                SignExtension::Unsigned,
            ) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(3u32, output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F32,
                SignExtension::Signed,
            ) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(4u32, output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F32,
                SignExtension::Unsigned,
            ) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(5u32, output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F64,
                SignExtension::Signed,
            ) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(6u32, output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F64,
                SignExtension::Unsigned,
            ) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(7u32, output)?;
            }
            _ => return Err(CompilerError::InvalidSyntax),
        }

        Ok(bytes)
    }
}

impl Emit for ReferenceInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::ReferenceNull(kind) => {
                bytes += emit_byte(0xD0u8, output)?;
                bytes += kind.emit(output)?;
            }
            Self::ReferenceIsNull => {
                bytes += emit_byte(0xD1u8, output)?;
            }
            Self::ReferenceFunction(index) => {
                bytes += emit_byte(0xD2u8, output)?;
                bytes += emit_usize(index, output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for ParametricInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::Drop => {
                bytes += emit_byte(0x1Au8, output)?;
            }
            Self::Select(Some(types)) => {
                bytes += emit_byte(0x1Cu8, output)?;
                bytes += types.emit(output)?;
            }
            Self::Select(None) => {
                bytes += emit_byte(0x1Bu8, output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for VariableInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::LocalGet(index) => {
                bytes += emit_byte(0x20u8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::LocalSet(index) => {
                bytes += emit_byte(0x21u8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::LocalTee(index) => {
                bytes += emit_byte(0x22u8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::GlobalGet(index) => {
                bytes += emit_byte(0x23u8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::GlobalSet(index) => {
                bytes += emit_byte(0x24u8, output)?;
                bytes += emit_usize(index, output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for TableInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::TableGet(index) => {
                bytes += emit_byte(0x25u8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::TableSet(index) => {
                bytes += emit_byte(0x26u8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::TableInit(element, table) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(12u32, output)?;
                bytes += emit_usize(element, output)?;
                bytes += emit_usize(table, output)?;
            }
            Self::ElementDrop(index) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(13u32, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::TableCopy(table_a, table_b) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(14u32, output)?;
                bytes += emit_usize(table_a, output)?;
                bytes += emit_usize(table_b, output)?;
            }
            Self::TableGrow(index) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(15u32, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::TableSize(index) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(16u32, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::TableFill(index) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(17u32, output)?;
                bytes += emit_usize(index, output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for MemoryInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::Load(NumberType::I32, memory_argument) => {
                bytes += emit_byte(0x28u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::I64, memory_argument) => {
                bytes += emit_byte(0x29u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::F32, memory_argument) => {
                bytes += emit_byte(0x2Au8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::F64, memory_argument) => {
                bytes += emit_byte(0x2Bu8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_8, SignExtension::Signed, memory_argument) => {
                bytes += emit_byte(0x2Cu8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_8, SignExtension::Unsigned, memory_argument) => {
                bytes += emit_byte(0x2Du8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_16, SignExtension::Signed, memory_argument) => {
                bytes += emit_byte(0x2Eu8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_16, SignExtension::Unsigned, memory_argument) => {
                bytes += emit_byte(0x2Fu8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_8, SignExtension::Signed, memory_argument) => {
                bytes += emit_byte(0x30u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_8, SignExtension::Unsigned, memory_argument) => {
                bytes += emit_byte(0x31u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_16, SignExtension::Signed, memory_argument) => {
                bytes += emit_byte(0x32u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_16, SignExtension::Unsigned, memory_argument) => {
                bytes += emit_byte(0x33u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_32, SignExtension::Signed, memory_argument) => {
                bytes += emit_byte(0x34u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_32, SignExtension::Unsigned, memory_argument) => {
                bytes += emit_byte(0x35u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::I32, memory_argument) => {
                bytes += emit_byte(0x36u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::I64, memory_argument) => {
                bytes += emit_byte(0x37u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::F32, memory_argument) => {
                bytes += emit_byte(0x38u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::F64, memory_argument) => {
                bytes += emit_byte(0x39u8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I32_8, memory_argument) => {
                bytes += emit_byte(0x3Au8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I32_16, memory_argument) => {
                bytes += emit_byte(0x3Bu8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_8, memory_argument) => {
                bytes += emit_byte(0x3Cu8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_16, memory_argument) => {
                bytes += emit_byte(0x3Du8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_32, memory_argument) => {
                bytes += emit_byte(0x3Eu8, output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::MemorySize => {
                bytes += emit_byte(0x3Fu8, output)?;
                bytes += emit_byte(0x00u8, output)?;
            }
            Self::MemoryGrow => {
                bytes += emit_byte(0x40u8, output)?;
                bytes += emit_byte(0x00u8, output)?;
            }
            Self::MemoryInit(index) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(8u32, output)?;
                bytes += emit_usize(index, output)?;
                bytes += emit_byte(0x00u8, output)?;
            }
            Self::DataDrop(index) => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(9u32, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::MemoryCopy => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(10u32, output)?;
                bytes += emit_byte(0x00u8, output)?;
                bytes += emit_byte(0x00u8, output)?;
            }
            Self::MemoryFill => {
                bytes += emit_byte(0xFCu8, output)?;
                bytes += emit_u32(11u32, output)?;
                bytes += emit_byte(0x00u8, output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for ControlInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::Unreachable => {
                bytes += emit_byte(0x00u8, output)?;
            }
            Self::Nop => {
                bytes += emit_byte(0x01u8, output)?;
            }
            Self::Block(kind, expression) => {
                bytes += emit_byte(0x02u8, output)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Self::Loop(kind, expression) => {
                bytes += emit_byte(0x03u8, output)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Self::If(kind, positive, negative) => {
                bytes += emit_byte(0x04u8, output)?;
                bytes += kind.emit(output)?;

                if let Some(negative) = negative {
                    for instruction in positive.instructions() {
                        bytes += instruction.emit(output)?;
                    }

                    bytes += emit_byte(0x05u8, output)?;
                    bytes += negative.emit(output)?;
                } else {
                    bytes += positive.emit(output)?;
                }
            }
            Self::Branch(index) => {
                bytes += emit_byte(0x0Cu8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::BranchIf(index) => {
                bytes += emit_byte(0x0Du8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::BranchTable(indices, index) => {
                bytes += emit_byte(0x0Eu8, output)?;
                bytes += emit_vector(indices, output, emit_usize)?;
                bytes += emit_usize(index, output)?;
            }
            Self::Return => {
                bytes += emit_byte(0x0Fu8, output)?;
            }
            Self::Call(index) => {
                bytes += emit_byte(0x10u8, output)?;
                bytes += emit_usize(index, output)?;
            }
            Self::CallIndirect(table, kind) => {
                bytes += emit_byte(0x11u8, output)?;
                bytes += emit_usize(kind, output)?;
                bytes += emit_usize(table, output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for BlockType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        match self {
            BlockType::Index(index) => (*index as i64).emit(output),
            BlockType::ValueType(kind) => kind.emit(output),
            BlockType::None => emit_byte(0x40u8, output),
        }
    }
}

impl Emit for MemoryArgument {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += emit_usize(self.align(), output)?;
        bytes += emit_usize(self.offset(), output)?;

        Ok(bytes)
    }
}
