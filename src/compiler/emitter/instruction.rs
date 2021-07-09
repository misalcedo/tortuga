use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{
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

        bytes += 0x0Bu8.emit(output)?;

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
                bytes += 0x41u8.emit(output)?;
                bytes += value.emit(output)?;
            }
            Self::I64Constant(value) => {
                bytes += 0x42u8.emit(output)?;
                bytes += value.emit(output)?;
            }
            Self::F32Constant(value) => {
                bytes += 0x43u8.emit(output)?;
                bytes += value.emit(output)?;
            }
            Self::F64Constant(value) => {
                bytes += 0x44u8.emit(output)?;
                bytes += value.emit(output)?;
            }
            // i32 Test Operations
            Self::EqualToZero(IntegerType::I32) => {
                bytes += 0x45u8.emit(output)?;
            }
            // i32 Relation Operations
            Self::Equal(NumberType::I32) => {
                bytes += 0x46u8.emit(output)?;
            }
            Self::NotEqual(NumberType::I32) => {
                bytes += 0x47u8.emit(output)?;
            }
            Self::LessThanInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += 0x48u8.emit(output)?;
            }
            Self::LessThanInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0x49u8.emit(output)?;
            }
            Self::GreaterThanInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += 0x4Au8.emit(output)?;
            }
            Self::GreaterThanInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0x4Bu8.emit(output)?;
            }
            Self::LessThanOrEqualToInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += 0x4Cu8.emit(output)?;
            }
            Self::LessThanOrEqualToInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0x4Du8.emit(output)?;
            }
            Self::GreaterThanOrEqualToInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += 0x4Eu8.emit(output)?;
            }
            Self::GreaterThanOrEqualToInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0x4Fu8.emit(output)?;
            }
            // i64 Test Operations
            Self::EqualToZero(IntegerType::I64) => {
                bytes += 0x50u8.emit(output)?;
            }
            // i64 Relation Operations
            Self::Equal(NumberType::I64) => {
                bytes += 0x51u8.emit(output)?;
            }
            Self::NotEqual(NumberType::I64) => {
                bytes += 0x52u8.emit(output)?;
            }
            Self::LessThanInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += 0x53u8.emit(output)?;
            }
            Self::LessThanInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0x54u8.emit(output)?;
            }
            Self::GreaterThanInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += 0x55u8.emit(output)?;
            }
            Self::GreaterThanInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0x56u8.emit(output)?;
            }
            Self::LessThanOrEqualToInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += 0x57u8.emit(output)?;
            }
            Self::LessThanOrEqualToInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0x58u8.emit(output)?;
            }
            Self::GreaterThanOrEqualToInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += 0x59u8.emit(output)?;
            }
            Self::GreaterThanOrEqualToInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0x5Au8.emit(output)?;
            }
            // f32 Relation Operations
            Self::Equal(NumberType::F32) => {
                bytes += 0x5Bu8.emit(output)?;
            }
            Self::NotEqual(NumberType::F32) => {
                bytes += 0x5Cu8.emit(output)?;
            }
            Self::LessThanFloat(FloatType::F32) => {
                bytes += 0x5Du8.emit(output)?;
            }
            Self::GreaterThanFloat(FloatType::F32) => {
                bytes += 0x5Eu8.emit(output)?;
            }
            Self::LessThanOrEqualToFloat(FloatType::F32) => {
                bytes += 0x5Fu8.emit(output)?;
            }
            Self::GreaterThanOrEqualToFloat(FloatType::F32) => {
                bytes += 0x60u8.emit(output)?;
            }
            // f64 Relation Operations
            Self::Equal(NumberType::F64) => {
                bytes += 0x61u8.emit(output)?;
            }
            Self::NotEqual(NumberType::F64) => {
                bytes += 0x62u8.emit(output)?;
            }
            Self::LessThanFloat(FloatType::F64) => {
                bytes += 0x63u8.emit(output)?;
            }
            Self::GreaterThanFloat(FloatType::F64) => {
                bytes += 0x64u8.emit(output)?;
            }
            Self::LessThanOrEqualToFloat(FloatType::F64) => {
                bytes += 0x65u8.emit(output)?;
            }
            Self::GreaterThanOrEqualToFloat(FloatType::F64) => {
                bytes += 0x66u8.emit(output)?;
            }
            // i32 Unary Operations
            Self::CountLeadingZeros(IntegerType::I32) => {
                bytes += 0x67u8.emit(output)?;
            }
            Self::CountTrailingZeros(IntegerType::I32) => {
                bytes += 0x68u8.emit(output)?;
            }
            Self::CountOnes(IntegerType::I32) => {
                bytes += 0x69u8.emit(output)?;
            }
            // i32 Binary Operations
            Self::Add(NumberType::I32) => {
                bytes += 0x6Au8.emit(output)?;
            }
            Self::Subtract(NumberType::I32) => {
                bytes += 0x6Bu8.emit(output)?;
            }
            Self::Multiply(NumberType::I32) => {
                bytes += 0x6Cu8.emit(output)?;
            }
            Self::DivideInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += 0x6Du8.emit(output)?;
            }
            Self::DivideInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0x6Eu8.emit(output)?;
            }
            Self::Remainder(IntegerType::I32, SignExtension::Signed) => {
                bytes += 0x6Fu8.emit(output)?;
            }
            Self::Remainder(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0x70u8.emit(output)?;
            }
            Self::And(IntegerType::I32) => {
                bytes += 0x71u8.emit(output)?;
            }
            Self::Or(IntegerType::I32) => {
                bytes += 0x72u8.emit(output)?;
            }
            Self::Xor(IntegerType::I32) => {
                bytes += 0x73u8.emit(output)?;
            }
            Self::ShiftLeft(IntegerType::I32) => {
                bytes += 0x74u8.emit(output)?;
            }
            Self::ShiftRight(IntegerType::I32, SignExtension::Signed) => {
                bytes += 0x75u8.emit(output)?;
            }
            Self::ShiftRight(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0x76u8.emit(output)?;
            }
            Self::RotateLeft(IntegerType::I32) => {
                bytes += 0x77u8.emit(output)?;
            }
            Self::RotateRight(IntegerType::I32) => {
                bytes += 0x78u8.emit(output)?;
            }
            // i64 Unary Operations
            Self::CountLeadingZeros(IntegerType::I64) => {
                bytes += 0x79u8.emit(output)?;
            }
            Self::CountTrailingZeros(IntegerType::I64) => {
                bytes += 0x7Au8.emit(output)?;
            }
            Self::CountOnes(IntegerType::I64) => {
                bytes += 0x7Bu8.emit(output)?;
            }
            // i64 Binary Operations
            Self::Add(NumberType::I64) => {
                bytes += 0x7Cu8.emit(output)?;
            }
            Self::Subtract(NumberType::I64) => {
                bytes += 0x7Du8.emit(output)?;
            }
            Self::Multiply(NumberType::I64) => {
                bytes += 0x7Eu8.emit(output)?;
            }
            Self::DivideInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += 0x7Fu8.emit(output)?;
            }
            Self::DivideInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0x80u8.emit(output)?;
            }
            Self::Remainder(IntegerType::I64, SignExtension::Signed) => {
                bytes += 0x81u8.emit(output)?;
            }
            Self::Remainder(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0x82u8.emit(output)?;
            }
            Self::And(IntegerType::I64) => {
                bytes += 0x83u8.emit(output)?;
            }
            Self::Or(IntegerType::I64) => {
                bytes += 0x84u8.emit(output)?;
            }
            Self::Xor(IntegerType::I64) => {
                bytes += 0x85u8.emit(output)?;
            }
            Self::ShiftLeft(IntegerType::I64) => {
                bytes += 0x86u8.emit(output)?;
            }
            Self::ShiftRight(IntegerType::I64, SignExtension::Signed) => {
                bytes += 0x87u8.emit(output)?;
            }
            Self::ShiftRight(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0x88u8.emit(output)?;
            }
            Self::RotateLeft(IntegerType::I64) => {
                bytes += 0x89u8.emit(output)?;
            }
            Self::RotateRight(IntegerType::I64) => {
                bytes += 0x8Au8.emit(output)?;
            }
            // f32 Unary Operations
            Self::AbsoluteValue(FloatType::F32) => {
                bytes += 0x8Bu8.emit(output)?;
            }
            Self::Negate(FloatType::F32) => {
                bytes += 0x8Cu8.emit(output)?;
            }
            Self::Ceiling(FloatType::F32) => {
                bytes += 0x8Du8.emit(output)?;
            }
            Self::Floor(FloatType::F32) => {
                bytes += 0x8Eu8.emit(output)?;
            }
            Self::Truncate(FloatType::F32) => {
                bytes += 0x8Fu8.emit(output)?;
            }
            Self::Nearest(FloatType::F32) => {
                bytes += 0x90u8.emit(output)?;
            }
            Self::SquareRoot(FloatType::F32) => {
                bytes += 0x91u8.emit(output)?;
            }
            // f32 Binary Operations
            Self::Add(NumberType::F32) => {
                bytes += 0x92u8.emit(output)?;
            }
            Self::Subtract(NumberType::F32) => {
                bytes += 0x93u8.emit(output)?;
            }
            Self::Multiply(NumberType::F32) => {
                bytes += 0x94u8.emit(output)?;
            }
            Self::DivideFloat(FloatType::F32) => {
                bytes += 0x95u8.emit(output)?;
            }
            Self::Minimum(FloatType::F32) => {
                bytes += 0x96u8.emit(output)?;
            }
            Self::Maximum(FloatType::F32) => {
                bytes += 0x97u8.emit(output)?;
            }
            Self::CopySign(FloatType::F32) => {
                bytes += 0x98u8.emit(output)?;
            }
            // f64 Unary Operations
            Self::AbsoluteValue(FloatType::F64) => {
                bytes += 0x99u8.emit(output)?;
            }
            Self::Negate(FloatType::F64) => {
                bytes += 0x9Au8.emit(output)?;
            }
            Self::Ceiling(FloatType::F64) => {
                bytes += 0x9Bu8.emit(output)?;
            }
            Self::Floor(FloatType::F64) => {
                bytes += 0x9Cu8.emit(output)?;
            }
            Self::Truncate(FloatType::F64) => {
                bytes += 0x9Du8.emit(output)?;
            }
            Self::Nearest(FloatType::F64) => {
                bytes += 0x9Eu8.emit(output)?;
            }
            Self::SquareRoot(FloatType::F64) => {
                bytes += 0x9Fu8.emit(output)?;
            }
            // f64 Binary Operations
            Self::Add(NumberType::F64) => {
                bytes += 0xA0u8.emit(output)?;
            }
            Self::Subtract(NumberType::F64) => {
                bytes += 0xA1u8.emit(output)?;
            }
            Self::Multiply(NumberType::F64) => {
                bytes += 0xA2u8.emit(output)?;
            }
            Self::DivideFloat(FloatType::F64) => {
                bytes += 0xA3u8.emit(output)?;
            }
            Self::Minimum(FloatType::F64) => {
                bytes += 0xA4u8.emit(output)?;
            }
            Self::Maximum(FloatType::F64) => {
                bytes += 0xA5u8.emit(output)?;
            }
            Self::CopySign(FloatType::F64) => {
                bytes += 0xA6u8.emit(output)?;
            }
            // Convert Operations
            Self::Wrap => {
                bytes += 0xA7u8.emit(output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I32, FloatType::F32, SignExtension::Signed) => {
                bytes += 0xA8u8.emit(output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I32, FloatType::F32, SignExtension::Unsigned) => {
                bytes += 0xA9u8.emit(output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I32, FloatType::F64, SignExtension::Signed) => {
                bytes += 0xAAu8.emit(output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I32, FloatType::F64, SignExtension::Unsigned) => {
                bytes += 0xABu8.emit(output)?;
            }
            Self::ExtendWithSignExtension(SignExtension::Signed) => {
                bytes += 0xACu8.emit(output)?;
            }
            Self::ExtendWithSignExtension(SignExtension::Unsigned) => {
                bytes += 0xADu8.emit(output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I64, FloatType::F32, SignExtension::Signed) => {
                bytes += 0xAEu8.emit(output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I64, FloatType::F32, SignExtension::Unsigned) => {
                bytes += 0xAFu8.emit(output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I64, FloatType::F64, SignExtension::Signed) => {
                bytes += 0xB0u8.emit(output)?;
            }
            Self::ConvertAndTruncate(IntegerType::I64, FloatType::F64, SignExtension::Unsigned) => {
                bytes += 0xB1u8.emit(output)?;
            }
            Self::Convert(FloatType::F32, IntegerType::I32, SignExtension::Signed) => {
                bytes += 0xB2u8.emit(output)?;
            }
            Self::Convert(FloatType::F32, IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0xB3u8.emit(output)?;
            }
            Self::Convert(FloatType::F32, IntegerType::I64, SignExtension::Signed) => {
                bytes += 0xB4u8.emit(output)?;
            }
            Self::Convert(FloatType::F32, IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0xB5u8.emit(output)?;
            }
            Self::Demote => {
                bytes += 0xB6u8.emit(output)?;
            }
            Self::Convert(FloatType::F64, IntegerType::I32, SignExtension::Signed) => {
                bytes += 0xB7u8.emit(output)?;
            }
            Self::Convert(FloatType::F64, IntegerType::I32, SignExtension::Unsigned) => {
                bytes += 0xB8u8.emit(output)?;
            }
            Self::Convert(FloatType::F64, IntegerType::I64, SignExtension::Signed) => {
                bytes += 0xB9u8.emit(output)?;
            }
            Self::Convert(FloatType::F64, IntegerType::I64, SignExtension::Unsigned) => {
                bytes += 0xBAu8.emit(output)?;
            }
            Self::Promote => {
                bytes += 0xBBu8.emit(output)?;
            }
            Self::ReinterpretFloat(IntegerType::I32, FloatType::F32) => {
                bytes += 0xBCu8.emit(output)?;
            }
            Self::ReinterpretFloat(IntegerType::I64, FloatType::F64) => {
                bytes += 0xBDu8.emit(output)?;
            }
            Self::ReinterpretInteger(FloatType::F32, IntegerType::I32) => {
                bytes += 0xBEu8.emit(output)?;
            }
            Self::ReinterpretInteger(FloatType::F64, IntegerType::I64) => {
                bytes += 0xBFu8.emit(output)?;
            }
            Self::ExtendSigned(StorageSize::I32_8) => {
                bytes += 0xC0u8.emit(output)?;
            }
            Self::ExtendSigned(StorageSize::I32_16) => {
                bytes += 0xC1u8.emit(output)?;
            }
            Self::ExtendSigned(StorageSize::I64_8) => {
                bytes += 0xC2u8.emit(output)?;
            }
            Self::ExtendSigned(StorageSize::I64_16) => {
                bytes += 0xC3u8.emit(output)?;
            }
            Self::ExtendSigned(StorageSize::I64_32) => {
                bytes += 0xC4u8.emit(output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F32,
                SignExtension::Signed,
            ) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 0u32.emit(output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F32,
                SignExtension::Unsigned,
            ) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 1u32.emit(output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F64,
                SignExtension::Signed,
            ) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 2u32.emit(output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F64,
                SignExtension::Unsigned,
            ) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 3u32.emit(output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F32,
                SignExtension::Signed,
            ) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 4u32.emit(output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F32,
                SignExtension::Unsigned,
            ) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 5u32.emit(output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F64,
                SignExtension::Signed,
            ) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 6u32.emit(output)?;
            }
            Self::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F64,
                SignExtension::Unsigned,
            ) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 7u32.emit(output)?;
            }
        }

        Ok(bytes)
    }
}

impl Emit for ReferenceInstruction {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            Self::ReferenceNull(kind) => {
                bytes += 0xD0u8.emit(output)?;
                bytes += kind.emit(output)?;
            }
            Self::ReferenceIsNull => {
                bytes += 0xD1u8.emit(output)?;
            }
            Self::ReferenceFunction(index) => {
                bytes += 0xD2u8.emit(output)?;
                bytes += index.emit(output)?;
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
                bytes += 0x1Au8.emit(output)?;
            }
            Self::Select(Some(types)) => {
                bytes += 0x1Cu8.emit(output)?;
                bytes += types.emit(output)?;
            }
            Self::Select(None) => {
                bytes += 0x1Bu8.emit(output)?;
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
                bytes += 0x20u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::LocalSet(index) => {
                bytes += 0x21u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::LocalTee(index) => {
                bytes += 0x22u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::GlobalGet(index) => {
                bytes += 0x23u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::GlobalSet(index) => {
                bytes += 0x24u8.emit(output)?;
                bytes += index.emit(output)?;
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
                bytes += 0x25u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableSet(index) => {
                bytes += 0x26u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableInit(element, table) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 12u32.emit(output)?;
                bytes += element.emit(output)?;
                bytes += table.emit(output)?;
            }
            Self::ElementDrop(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 13u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableCopy(table_a, table_b) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 14u32.emit(output)?;
                bytes += table_a.emit(output)?;
                bytes += table_b.emit(output)?;
            }
            Self::TableGrow(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 15u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableSize(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 16u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::TableFill(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 17u32.emit(output)?;
                bytes += index.emit(output)?;
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
                bytes += 0x28u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::I64, memory_argument) => {
                bytes += 0x29u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::F32, memory_argument) => {
                bytes += 0x2Au8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Load(NumberType::F64, memory_argument) => {
                bytes += 0x2Bu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_8, SignExtension::Signed, memory_argument) => {
                bytes += 0x2Cu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_8, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x2Du8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_16, SignExtension::Signed, memory_argument) => {
                bytes += 0x2Eu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I32_16, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x2Fu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_8, SignExtension::Signed, memory_argument) => {
                bytes += 0x30u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_8, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x31u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_16, SignExtension::Signed, memory_argument) => {
                bytes += 0x32u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_16, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x33u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_32, SignExtension::Signed, memory_argument) => {
                bytes += 0x34u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::LoadPartial(StorageSize::I64_32, SignExtension::Unsigned, memory_argument) => {
                bytes += 0x35u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::I32, memory_argument) => {
                bytes += 0x36u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::I64, memory_argument) => {
                bytes += 0x37u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::F32, memory_argument) => {
                bytes += 0x38u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::Store(NumberType::F64, memory_argument) => {
                bytes += 0x39u8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I32_8, memory_argument) => {
                bytes += 0x3Au8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I32_16, memory_argument) => {
                bytes += 0x3Bu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_8, memory_argument) => {
                bytes += 0x3Cu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_16, memory_argument) => {
                bytes += 0x3Du8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::StorePartial(StorageSize::I64_32, memory_argument) => {
                bytes += 0x3Eu8.emit(output)?;
                bytes += memory_argument.emit(output)?;
            }
            Self::MemorySize => {
                bytes += 0x3Fu8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Self::MemoryGrow => {
                bytes += 0x40u8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Self::MemoryInit(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 8u32.emit(output)?;
                bytes += index.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Self::DataDrop(index) => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 9u32.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::MemoryCopy => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 10u32.emit(output)?;
                bytes += 0x00u8.emit(output)?;
                bytes += 0x00u8.emit(output)?;
            }
            Self::MemoryFill => {
                bytes += 0xFCu8.emit(output)?;
                bytes += 11u32.emit(output)?;
                bytes += 0x00u8.emit(output)?;
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
                bytes += 0x00u8.emit(output)?;
            }
            Self::Nop => {
                bytes += 0x01u8.emit(output)?;
            }
            Self::Block(kind, expression) => {
                bytes += 0x02u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Self::Loop(kind, expression) => {
                bytes += 0x03u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expression.emit(output)?;
            }
            Self::If(kind, positive, negative) => {
                bytes += 0x04u8.emit(output)?;
                bytes += kind.emit(output)?;

                if let Some(negative) = negative {
                    for instruction in positive.instructions() {
                        bytes += instruction.emit(output)?;
                    }

                    bytes += 0x05u8.emit(output)?;
                    bytes += negative.emit(output)?;
                } else {
                    bytes += positive.emit(output)?;
                }
            }
            Self::Branch(index) => {
                bytes += 0x0Cu8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::BranchIf(index) => {
                bytes += 0x0Du8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::BranchTable(indices, index) => {
                bytes += 0x0Eu8.emit(output)?;
                bytes += indices.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::Return => {
                bytes += 0x0Fu8.emit(output)?;
            }
            Self::Call(index) => {
                bytes += 0x10u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            Self::CallIndirect(table, kind) => {
                bytes += 0x11u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += table.emit(output)?;
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
            BlockType::None => 0x40u8.emit(output),
        }
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
