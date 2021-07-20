use crate::compiler::emitter::BinaryEmitter;
use crate::compiler::CompilerError;
use crate::syntax::web_assembly::{
    BlockType, ControlInstruction, Expression, FloatType, Instruction, IntegerType, MemoryArgument,
    MemoryInstruction, NumberType, NumericInstruction, ParametricInstruction, ReferenceInstruction,
    SignExtension, StorageSize, TableInstruction, VariableInstruction,
};
use futures::AsyncWrite;

impl<'output, O: AsyncWrite + Unpin> BinaryEmitter<'output, O> {
    pub async fn emit_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<usize, CompilerError> {
        self.emit_expression_with_custom_terminator(expression, 0x0B)
            .await
    }

    async fn emit_expression_with_custom_terminator(
        &mut self,
        expression: &Expression,
        terminator: u8,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        for instruction in expression.instructions() {
            bytes += self.emit_instruction(instruction).await?;
        }

        bytes += self.emit_u8(terminator).await?;

        Ok(bytes)
    }

    pub async fn emit_instruction(
        &mut self,
        instruction: &Instruction,
    ) -> Result<usize, CompilerError> {
        match instruction {
            Instruction::Numeric(instruction) => self.emit_numeric_instruction(instruction).await,
            Instruction::Reference(instruction) => {
                self.emit_reference_instruction(instruction).await
            }
            Instruction::Parametric(instruction) => {
                self.emit_parametric_instruction(instruction).await
            }
            Instruction::Variable(instruction) => self.emit_variable_instruction(instruction).await,
            Instruction::Table(instruction) => self.emit_table_instruction(instruction).await,
            Instruction::Memory(instruction) => self.emit_memory_instruction(instruction).await,
            Instruction::Control(instruction) => self.emit_control_instruction(instruction).await,
        }
    }

    pub async fn emit_numeric_instruction(
        &mut self,
        instruction: &NumericInstruction,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match instruction {
            // Constant Operations
            NumericInstruction::I32Constant(value) => {
                bytes += self.emit_u8(0x41).await?;
                bytes += self.emit_i32(*value).await?;
            }
            NumericInstruction::I64Constant(value) => {
                bytes += self.emit_u8(0x42).await?;
                bytes += self.emit_i64(*value).await?;
            }
            NumericInstruction::F32Constant(value) => {
                bytes += self.emit_u8(0x43).await?;
                bytes += self.emit_f32(*value).await?;
            }
            NumericInstruction::F64Constant(value) => {
                bytes += self.emit_u8(0x44).await?;
                bytes += self.emit_f64(*value).await?;
            }
            // i32 Test Operations
            NumericInstruction::EqualToZero(IntegerType::I32) => {
                bytes += self.emit_u8(0x45).await?;
            }
            // i32 Relation Operations
            NumericInstruction::Equal(NumberType::I32) => {
                bytes += self.emit_u8(0x46).await?;
            }
            NumericInstruction::NotEqual(NumberType::I32) => {
                bytes += self.emit_u8(0x47).await?;
            }
            NumericInstruction::LessThanInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += self.emit_u8(0x48).await?;
            }
            NumericInstruction::LessThanInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x49).await?;
            }
            NumericInstruction::GreaterThanInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += self.emit_u8(0x4A).await?;
            }
            NumericInstruction::GreaterThanInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x4B).await?;
            }
            NumericInstruction::LessThanOrEqualToInteger(
                IntegerType::I32,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0x4C).await?;
            }
            NumericInstruction::LessThanOrEqualToInteger(
                IntegerType::I32,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0x4D).await?;
            }
            NumericInstruction::GreaterThanOrEqualToInteger(
                IntegerType::I32,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0x4E).await?;
            }
            NumericInstruction::GreaterThanOrEqualToInteger(
                IntegerType::I32,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0x4F).await?;
            }
            // i64 Test Operations
            NumericInstruction::EqualToZero(IntegerType::I64) => {
                bytes += self.emit_u8(0x50).await?;
            }
            // i64 Relation Operations
            NumericInstruction::Equal(NumberType::I64) => {
                bytes += self.emit_u8(0x51).await?;
            }
            NumericInstruction::NotEqual(NumberType::I64) => {
                bytes += self.emit_u8(0x52).await?;
            }
            NumericInstruction::LessThanInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += self.emit_u8(0x53).await?;
            }
            NumericInstruction::LessThanInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x54).await?;
            }
            NumericInstruction::GreaterThanInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += self.emit_u8(0x55).await?;
            }
            NumericInstruction::GreaterThanInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x56).await?;
            }
            NumericInstruction::LessThanOrEqualToInteger(
                IntegerType::I64,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0x57).await?;
            }
            NumericInstruction::LessThanOrEqualToInteger(
                IntegerType::I64,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0x58).await?;
            }
            NumericInstruction::GreaterThanOrEqualToInteger(
                IntegerType::I64,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0x59).await?;
            }
            NumericInstruction::GreaterThanOrEqualToInteger(
                IntegerType::I64,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0x5A).await?;
            }
            // f32 Relation Operations
            NumericInstruction::Equal(NumberType::F32) => {
                bytes += self.emit_u8(0x5B).await?;
            }
            NumericInstruction::NotEqual(NumberType::F32) => {
                bytes += self.emit_u8(0x5C).await?;
            }
            NumericInstruction::LessThanFloat(FloatType::F32) => {
                bytes += self.emit_u8(0x5D).await?;
            }
            NumericInstruction::GreaterThanFloat(FloatType::F32) => {
                bytes += self.emit_u8(0x5E).await?;
            }
            NumericInstruction::LessThanOrEqualToFloat(FloatType::F32) => {
                bytes += self.emit_u8(0x5F).await?;
            }
            NumericInstruction::GreaterThanOrEqualToFloat(FloatType::F32) => {
                bytes += self.emit_u8(0x60).await?;
            }
            // f64 Relation Operations
            NumericInstruction::Equal(NumberType::F64) => {
                bytes += self.emit_u8(0x61).await?;
            }
            NumericInstruction::NotEqual(NumberType::F64) => {
                bytes += self.emit_u8(0x62).await?;
            }
            NumericInstruction::LessThanFloat(FloatType::F64) => {
                bytes += self.emit_u8(0x63).await?;
            }
            NumericInstruction::GreaterThanFloat(FloatType::F64) => {
                bytes += self.emit_u8(0x64).await?;
            }
            NumericInstruction::LessThanOrEqualToFloat(FloatType::F64) => {
                bytes += self.emit_u8(0x65).await?;
            }
            NumericInstruction::GreaterThanOrEqualToFloat(FloatType::F64) => {
                bytes += self.emit_u8(0x66).await?;
            }
            // i32 Unary Operations
            NumericInstruction::CountLeadingZeros(IntegerType::I32) => {
                bytes += self.emit_u8(0x67).await?;
            }
            NumericInstruction::CountTrailingZeros(IntegerType::I32) => {
                bytes += self.emit_u8(0x68).await?;
            }
            NumericInstruction::CountOnes(IntegerType::I32) => {
                bytes += self.emit_u8(0x69).await?;
            }
            // i32 Binary Operations
            NumericInstruction::Add(NumberType::I32) => {
                bytes += self.emit_u8(0x6A).await?;
            }
            NumericInstruction::Subtract(NumberType::I32) => {
                bytes += self.emit_u8(0x6B).await?;
            }
            NumericInstruction::Multiply(NumberType::I32) => {
                bytes += self.emit_u8(0x6C).await?;
            }
            NumericInstruction::DivideInteger(IntegerType::I32, SignExtension::Signed) => {
                bytes += self.emit_u8(0x6D).await?;
            }
            NumericInstruction::DivideInteger(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x6E).await?;
            }
            NumericInstruction::Remainder(IntegerType::I32, SignExtension::Signed) => {
                bytes += self.emit_u8(0x6F).await?;
            }
            NumericInstruction::Remainder(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x70).await?;
            }
            NumericInstruction::And(IntegerType::I32) => {
                bytes += self.emit_u8(0x71).await?;
            }
            NumericInstruction::Or(IntegerType::I32) => {
                bytes += self.emit_u8(0x72).await?;
            }
            NumericInstruction::Xor(IntegerType::I32) => {
                bytes += self.emit_u8(0x73).await?;
            }
            NumericInstruction::ShiftLeft(IntegerType::I32) => {
                bytes += self.emit_u8(0x74).await?;
            }
            NumericInstruction::ShiftRight(IntegerType::I32, SignExtension::Signed) => {
                bytes += self.emit_u8(0x75).await?;
            }
            NumericInstruction::ShiftRight(IntegerType::I32, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x76).await?;
            }
            NumericInstruction::RotateLeft(IntegerType::I32) => {
                bytes += self.emit_u8(0x77).await?;
            }
            NumericInstruction::RotateRight(IntegerType::I32) => {
                bytes += self.emit_u8(0x78).await?;
            }
            // i64 Unary Operations
            NumericInstruction::CountLeadingZeros(IntegerType::I64) => {
                bytes += self.emit_u8(0x79).await?;
            }
            NumericInstruction::CountTrailingZeros(IntegerType::I64) => {
                bytes += self.emit_u8(0x7A).await?;
            }
            NumericInstruction::CountOnes(IntegerType::I64) => {
                bytes += self.emit_u8(0x7B).await?;
            }
            // i64 Binary Operations
            NumericInstruction::Add(NumberType::I64) => {
                bytes += self.emit_u8(0x7C).await?;
            }
            NumericInstruction::Subtract(NumberType::I64) => {
                bytes += self.emit_u8(0x7D).await?;
            }
            NumericInstruction::Multiply(NumberType::I64) => {
                bytes += self.emit_u8(0x7E).await?;
            }
            NumericInstruction::DivideInteger(IntegerType::I64, SignExtension::Signed) => {
                bytes += self.emit_u8(0x7F).await?;
            }
            NumericInstruction::DivideInteger(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x80).await?;
            }
            NumericInstruction::Remainder(IntegerType::I64, SignExtension::Signed) => {
                bytes += self.emit_u8(0x81).await?;
            }
            NumericInstruction::Remainder(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x82).await?;
            }
            NumericInstruction::And(IntegerType::I64) => {
                bytes += self.emit_u8(0x83).await?;
            }
            NumericInstruction::Or(IntegerType::I64) => {
                bytes += self.emit_u8(0x84).await?;
            }
            NumericInstruction::Xor(IntegerType::I64) => {
                bytes += self.emit_u8(0x85).await?;
            }
            NumericInstruction::ShiftLeft(IntegerType::I64) => {
                bytes += self.emit_u8(0x86).await?;
            }
            NumericInstruction::ShiftRight(IntegerType::I64, SignExtension::Signed) => {
                bytes += self.emit_u8(0x87).await?;
            }
            NumericInstruction::ShiftRight(IntegerType::I64, SignExtension::Unsigned) => {
                bytes += self.emit_u8(0x88).await?;
            }
            NumericInstruction::RotateLeft(IntegerType::I64) => {
                bytes += self.emit_u8(0x89).await?;
            }
            NumericInstruction::RotateRight(IntegerType::I64) => {
                bytes += self.emit_u8(0x8A).await?;
            }
            // f32 Unary Operations
            NumericInstruction::AbsoluteValue(FloatType::F32) => {
                bytes += self.emit_u8(0x8B).await?;
            }
            NumericInstruction::Negate(FloatType::F32) => {
                bytes += self.emit_u8(0x8C).await?;
            }
            NumericInstruction::Ceiling(FloatType::F32) => {
                bytes += self.emit_u8(0x8D).await?;
            }
            NumericInstruction::Floor(FloatType::F32) => {
                bytes += self.emit_u8(0x8E).await?;
            }
            NumericInstruction::Truncate(FloatType::F32) => {
                bytes += self.emit_u8(0x8F).await?;
            }
            NumericInstruction::Nearest(FloatType::F32) => {
                bytes += self.emit_u8(0x90).await?;
            }
            NumericInstruction::SquareRoot(FloatType::F32) => {
                bytes += self.emit_u8(0x91).await?;
            }
            // f32 Binary Operations
            NumericInstruction::Add(NumberType::F32) => {
                bytes += self.emit_u8(0x92).await?;
            }
            NumericInstruction::Subtract(NumberType::F32) => {
                bytes += self.emit_u8(0x93).await?;
            }
            NumericInstruction::Multiply(NumberType::F32) => {
                bytes += self.emit_u8(0x94).await?;
            }
            NumericInstruction::DivideFloat(FloatType::F32) => {
                bytes += self.emit_u8(0x95).await?;
            }
            NumericInstruction::Minimum(FloatType::F32) => {
                bytes += self.emit_u8(0x96).await?;
            }
            NumericInstruction::Maximum(FloatType::F32) => {
                bytes += self.emit_u8(0x97).await?;
            }
            NumericInstruction::CopySign(FloatType::F32) => {
                bytes += self.emit_u8(0x98).await?;
            }
            // f64 Unary Operations
            NumericInstruction::AbsoluteValue(FloatType::F64) => {
                bytes += self.emit_u8(0x99).await?;
            }
            NumericInstruction::Negate(FloatType::F64) => {
                bytes += self.emit_u8(0x9A).await?;
            }
            NumericInstruction::Ceiling(FloatType::F64) => {
                bytes += self.emit_u8(0x9B).await?;
            }
            NumericInstruction::Floor(FloatType::F64) => {
                bytes += self.emit_u8(0x9C).await?;
            }
            NumericInstruction::Truncate(FloatType::F64) => {
                bytes += self.emit_u8(0x9D).await?;
            }
            NumericInstruction::Nearest(FloatType::F64) => {
                bytes += self.emit_u8(0x9E).await?;
            }
            NumericInstruction::SquareRoot(FloatType::F64) => {
                bytes += self.emit_u8(0x9F).await?;
            }
            // f64 Binary Operations
            NumericInstruction::Add(NumberType::F64) => {
                bytes += self.emit_u8(0xA0).await?;
            }
            NumericInstruction::Subtract(NumberType::F64) => {
                bytes += self.emit_u8(0xA1).await?;
            }
            NumericInstruction::Multiply(NumberType::F64) => {
                bytes += self.emit_u8(0xA2).await?;
            }
            NumericInstruction::DivideFloat(FloatType::F64) => {
                bytes += self.emit_u8(0xA3).await?;
            }
            NumericInstruction::Minimum(FloatType::F64) => {
                bytes += self.emit_u8(0xA4).await?;
            }
            NumericInstruction::Maximum(FloatType::F64) => {
                bytes += self.emit_u8(0xA5).await?;
            }
            NumericInstruction::CopySign(FloatType::F64) => {
                bytes += self.emit_u8(0xA6).await?;
            }
            // Convert Operations
            NumericInstruction::Wrap => {
                bytes += self.emit_u8(0xA7).await?;
            }
            NumericInstruction::ConvertAndTruncate(
                IntegerType::I32,
                FloatType::F32,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xA8).await?;
            }
            NumericInstruction::ConvertAndTruncate(
                IntegerType::I32,
                FloatType::F32,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xA9).await?;
            }
            NumericInstruction::ConvertAndTruncate(
                IntegerType::I32,
                FloatType::F64,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xAA).await?;
            }
            NumericInstruction::ConvertAndTruncate(
                IntegerType::I32,
                FloatType::F64,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xAB).await?;
            }
            NumericInstruction::ExtendWithSignExtension(SignExtension::Signed) => {
                bytes += self.emit_u8(0xAC).await?;
            }
            NumericInstruction::ExtendWithSignExtension(SignExtension::Unsigned) => {
                bytes += self.emit_u8(0xAD).await?;
            }
            NumericInstruction::ConvertAndTruncate(
                IntegerType::I64,
                FloatType::F32,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xAE).await?;
            }
            NumericInstruction::ConvertAndTruncate(
                IntegerType::I64,
                FloatType::F32,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xAF).await?;
            }
            NumericInstruction::ConvertAndTruncate(
                IntegerType::I64,
                FloatType::F64,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xB0).await?;
            }
            NumericInstruction::ConvertAndTruncate(
                IntegerType::I64,
                FloatType::F64,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xB1).await?;
            }
            NumericInstruction::Convert(
                FloatType::F32,
                IntegerType::I32,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xB2).await?;
            }
            NumericInstruction::Convert(
                FloatType::F32,
                IntegerType::I32,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xB3).await?;
            }
            NumericInstruction::Convert(
                FloatType::F32,
                IntegerType::I64,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xB4).await?;
            }
            NumericInstruction::Convert(
                FloatType::F32,
                IntegerType::I64,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xB5).await?;
            }
            NumericInstruction::Demote => {
                bytes += self.emit_u8(0xB6).await?;
            }
            NumericInstruction::Convert(
                FloatType::F64,
                IntegerType::I32,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xB7).await?;
            }
            NumericInstruction::Convert(
                FloatType::F64,
                IntegerType::I32,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xB8).await?;
            }
            NumericInstruction::Convert(
                FloatType::F64,
                IntegerType::I64,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xB9).await?;
            }
            NumericInstruction::Convert(
                FloatType::F64,
                IntegerType::I64,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xBA).await?;
            }
            NumericInstruction::Promote => {
                bytes += self.emit_u8(0xBB).await?;
            }
            NumericInstruction::ReinterpretFloat(IntegerType::I32, FloatType::F32) => {
                bytes += self.emit_u8(0xBC).await?;
            }
            NumericInstruction::ReinterpretFloat(IntegerType::I64, FloatType::F64) => {
                bytes += self.emit_u8(0xBD).await?;
            }
            NumericInstruction::ReinterpretInteger(FloatType::F32, IntegerType::I32) => {
                bytes += self.emit_u8(0xBE).await?;
            }
            NumericInstruction::ReinterpretInteger(FloatType::F64, IntegerType::I64) => {
                bytes += self.emit_u8(0xBF).await?;
            }
            NumericInstruction::ExtendSigned(StorageSize::I32_8) => {
                bytes += self.emit_u8(0xC0).await?;
            }
            NumericInstruction::ExtendSigned(StorageSize::I32_16) => {
                bytes += self.emit_u8(0xC1).await?;
            }
            NumericInstruction::ExtendSigned(StorageSize::I64_8) => {
                bytes += self.emit_u8(0xC2).await?;
            }
            NumericInstruction::ExtendSigned(StorageSize::I64_16) => {
                bytes += self.emit_u8(0xC3).await?;
            }
            NumericInstruction::ExtendSigned(StorageSize::I64_32) => {
                bytes += self.emit_u8(0xC4).await?;
            }
            NumericInstruction::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F32,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(0).await?;
            }
            NumericInstruction::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F32,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(1).await?;
            }
            NumericInstruction::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F64,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(2).await?;
            }
            NumericInstruction::ConvertAndTruncateWithSaturation(
                IntegerType::I32,
                FloatType::F64,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(3).await?;
            }
            NumericInstruction::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F32,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(4).await?;
            }
            NumericInstruction::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F32,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(5).await?;
            }
            NumericInstruction::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F64,
                SignExtension::Signed,
            ) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(6).await?;
            }
            NumericInstruction::ConvertAndTruncateWithSaturation(
                IntegerType::I64,
                FloatType::F64,
                SignExtension::Unsigned,
            ) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(7).await?;
            }
            _ => return Err(CompilerError::InvalidSyntax),
        }

        Ok(bytes)
    }

    pub async fn emit_reference_instruction(
        &mut self,
        instruction: &ReferenceInstruction,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match instruction {
            ReferenceInstruction::ReferenceNull(kind) => {
                bytes += self.emit_u8(0xD0).await?;
                bytes += self.emit_reference_type(kind).await?;
            }
            ReferenceInstruction::ReferenceIsNull => {
                bytes += self.emit_u8(0xD1).await?;
            }
            ReferenceInstruction::ReferenceFunction(index) => {
                bytes += self.emit_u8(0xD2).await?;
                bytes += self.emit_usize(*index).await?;
            }
        }

        Ok(bytes)
    }

    pub async fn emit_parametric_instruction(
        &mut self,
        instruction: &ParametricInstruction,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match instruction {
            ParametricInstruction::Drop => {
                bytes += self.emit_u8(0x1A).await?;
            }
            ParametricInstruction::Select(Some(types)) => {
                bytes += self.emit_u8(0x1C).await?;
                bytes += self.emit_vector(types, Self::emit_value_type).await?;
            }
            ParametricInstruction::Select(None) => {
                bytes += self.emit_u8(0x1B).await?;
            }
        }

        Ok(bytes)
    }

    pub async fn emit_variable_instruction(
        &mut self,
        instruction: &VariableInstruction,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match instruction {
            VariableInstruction::LocalGet(index) => {
                bytes += self.emit_u8(0x20).await?;
                bytes += self.emit_usize(*index).await?;
            }
            VariableInstruction::LocalSet(index) => {
                bytes += self.emit_u8(0x21).await?;
                bytes += self.emit_usize(*index).await?;
            }
            VariableInstruction::LocalTee(index) => {
                bytes += self.emit_u8(0x22).await?;
                bytes += self.emit_usize(*index).await?;
            }
            VariableInstruction::GlobalGet(index) => {
                bytes += self.emit_u8(0x23).await?;
                bytes += self.emit_usize(*index).await?;
            }
            VariableInstruction::GlobalSet(index) => {
                bytes += self.emit_u8(0x24).await?;
                bytes += self.emit_usize(*index).await?;
            }
        }

        Ok(bytes)
    }

    pub async fn emit_table_instruction(
        &mut self,
        instruction: &TableInstruction,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match instruction {
            TableInstruction::TableGet(index) => {
                bytes += self.emit_u8(0x25).await?;
                bytes += self.emit_usize(*index).await?;
            }
            TableInstruction::TableSet(index) => {
                bytes += self.emit_u8(0x26).await?;
                bytes += self.emit_usize(*index).await?;
            }
            TableInstruction::TableInit(element, table) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(12).await?;
                bytes += self.emit_usize(*element).await?;
                bytes += self.emit_usize(*table).await?;
            }
            TableInstruction::ElementDrop(index) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(13).await?;
                bytes += self.emit_usize(*index).await?;
            }
            TableInstruction::TableCopy(table_a, table_b) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(14).await?;
                bytes += self.emit_usize(*table_a).await?;
                bytes += self.emit_usize(*table_b).await?;
            }
            TableInstruction::TableGrow(index) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(15).await?;
                bytes += self.emit_usize(*index).await?;
            }
            TableInstruction::TableSize(index) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(16).await?;
                bytes += self.emit_usize(*index).await?;
            }
            TableInstruction::TableFill(index) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(17).await?;
                bytes += self.emit_usize(*index).await?;
            }
        }

        Ok(bytes)
    }

    pub async fn emit_memory_instruction(
        &mut self,
        instruction: &MemoryInstruction,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match instruction {
            MemoryInstruction::Load(NumberType::I32, memory_argument) => {
                bytes += self.emit_u8(0x28).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::Load(NumberType::I64, memory_argument) => {
                bytes += self.emit_u8(0x29).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::Load(NumberType::F32, memory_argument) => {
                bytes += self.emit_u8(0x2A).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::Load(NumberType::F64, memory_argument) => {
                bytes += self.emit_u8(0x2B).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I32_8,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x2C).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I32_8,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x2D).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I32_16,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x2E).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I32_16,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x2F).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I64_8,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x30).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I64_8,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x31).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I64_16,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x32).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I64_16,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x33).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I64_32,
                SignExtension::Signed,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x34).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::LoadPartial(
                StorageSize::I64_32,
                SignExtension::Unsigned,
                memory_argument,
            ) => {
                bytes += self.emit_u8(0x35).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::Store(NumberType::I32, memory_argument) => {
                bytes += self.emit_u8(0x36).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::Store(NumberType::I64, memory_argument) => {
                bytes += self.emit_u8(0x37).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::Store(NumberType::F32, memory_argument) => {
                bytes += self.emit_u8(0x38).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::Store(NumberType::F64, memory_argument) => {
                bytes += self.emit_u8(0x39).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::StorePartial(StorageSize::I32_8, memory_argument) => {
                bytes += self.emit_u8(0x3A).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::StorePartial(StorageSize::I32_16, memory_argument) => {
                bytes += self.emit_u8(0x3B).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::StorePartial(StorageSize::I64_8, memory_argument) => {
                bytes += self.emit_u8(0x3C).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::StorePartial(StorageSize::I64_16, memory_argument) => {
                bytes += self.emit_u8(0x3D).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::StorePartial(StorageSize::I64_32, memory_argument) => {
                bytes += self.emit_u8(0x3E).await?;
                bytes += self.emit_memory_argument(memory_argument).await?;
            }
            MemoryInstruction::MemorySize => {
                bytes += self.emit_u8(0x3F).await?;
                bytes += self.emit_u8(0x00).await?;
            }
            MemoryInstruction::MemoryGrow => {
                bytes += self.emit_u8(0x40).await?;
                bytes += self.emit_u8(0x00).await?;
            }
            MemoryInstruction::MemoryInit(index) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(8).await?;
                bytes += self.emit_usize(*index).await?;
                bytes += self.emit_u8(0x00).await?;
            }
            MemoryInstruction::DataDrop(index) => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(9).await?;
                bytes += self.emit_usize(*index).await?;
            }
            MemoryInstruction::MemoryCopy => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(10).await?;
                bytes += self.emit_u8(0x00).await?;
                bytes += self.emit_u8(0x00).await?;
            }
            MemoryInstruction::MemoryFill => {
                bytes += self.emit_u8(0xFC).await?;
                bytes += self.emit_u32(11).await?;
                bytes += self.emit_u8(0x00).await?;
            }
        }

        Ok(bytes)
    }

    pub async fn emit_control_instruction(
        &mut self,
        instruction: &ControlInstruction,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match instruction {
            ControlInstruction::Unreachable => {
                bytes += self.emit_u8(0x00).await?;
            }
            ControlInstruction::Nop => {
                bytes += self.emit_u8(0x01).await?;
            }
            ControlInstruction::Block(kind, expression) => {
                bytes += self.emit_u8(0x02).await?;
                bytes += self.emit_block_type(kind).await?;
                bytes += self.emit_expression(expression).await?;
            }
            ControlInstruction::Loop(kind, expression) => {
                bytes += self.emit_u8(0x03).await?;
                bytes += self.emit_block_type(kind).await?;
                bytes += self.emit_expression(expression).await?;
            }
            ControlInstruction::If(kind, positive, negative) => {
                bytes += self.emit_u8(0x04).await?;
                bytes += self.emit_block_type(kind).await?;

                if let Some(negative) = negative {
                    bytes += self
                        .emit_expression_with_custom_terminator(negative, 0x05)
                        .await?;
                    bytes += self.emit_expression(negative).await?;
                } else {
                    bytes += self.emit_expression(positive).await?;
                }
            }
            ControlInstruction::Branch(index) => {
                bytes += self.emit_u8(0x0C).await?;
                bytes += self.emit_usize(*index).await?;
            }
            ControlInstruction::BranchIf(index) => {
                bytes += self.emit_u8(0x0D).await?;
                bytes += self.emit_usize(*index).await?;
            }
            ControlInstruction::BranchTable(indices, index) => {
                bytes += self.emit_u8(0x0E).await?;
                bytes += self.emit_vector(indices, Self::emit_usize).await?;
                bytes += self.emit_usize(index).await?;
            }
            ControlInstruction::Return => {
                bytes += self.emit_u8(0x0F).await?;
            }
            ControlInstruction::Call(index) => {
                bytes += self.emit_u8(0x10).await?;
                bytes += self.emit_usize(*index).await?;
            }
            ControlInstruction::CallIndirect(table, kind) => {
                bytes += self.emit_u8(0x11).await?;
                bytes += self.emit_usize(*kind).await?;
                bytes += self.emit_usize(*table).await?;
            }
        }

        Ok(bytes)
    }

    pub async fn emit_block_type(&mut self, value: &BlockType) -> Result<usize, CompilerError> {
        match value {
            BlockType::Index(index) => self.emit_i64(*index as i64).await,
            BlockType::ValueType(kind) => self.emit_value_type(kind).await,
            BlockType::None => self.emit_u8(0x40).await,
        }
    }

    pub async fn emit_memory_argument(
        &mut self,
        value: &MemoryArgument,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_usize(value.align()).await?;
        bytes += self.emit_usize(value.offset()).await?;

        Ok(bytes)
    }
}
