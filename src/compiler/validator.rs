use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly;
use std::error::Error;
use wasmtime::{Engine, Instance, Module, Store};

/// A validation of a WebAssembly module.
pub trait Validate<T: Error> {
    /// Tests that the module is well-formed. The meaning of well-formed -ness is specific to the
    /// trait implementation.
    fn validate(module: web_assembly::Module) -> Result<(), T>;
}

pub struct SyntaxCheck {}

impl Validate<CompilerError> for SyntaxCheck {
    fn validate(target: web_assembly::Module) -> Result<(), CompilerError> {
        let mut bytes = Vec::new();

        target.emit(&mut bytes)?;

        let engine = Engine::default();
        let module = Module::new(&engine, &bytes)?;
        let mut store = Store::new(&engine, 0);

        Instance::new(&mut store, &module, &[])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web_assembly::{
        ControlInstruction, Data, DataMode, Element, ElementInitializer, ElementMode, Export,
        ExportDescription, Expression, Function, FunctionType, Global, GlobalType, Instruction,
        Limit, Memory, MemoryType, Name, NumberType, NumericInstruction, ReferenceType, ResultType,
        Start, Table, TableType, ValueType,
    };

    #[test]
    fn valid_empty_module() {
        let module = web_assembly::Module::new();
        let result = SyntaxCheck::validate(module);

        assert!(result.is_ok());
    }

    #[test]
    fn valid_module() {
        let mut module = web_assembly::Module::new();
        let function_type = FunctionType::new(
            ResultType::new(vec![ValueType::Number(NumberType::I64)]),
            ResultType::new(vec![ValueType::Number(NumberType::F64)]),
        );
        module.add_type(function_type);

        let function = Function::new(
            0,
            ResultType::new(vec![ValueType::Number(NumberType::I32)]),
            Expression::new(vec![Instruction::Numeric(NumericInstruction::F64Constant(
                0.0,
            ))]),
        );
        module.add_function(function);

        let start_function_type =
            FunctionType::new(ResultType::new(vec![]), ResultType::new(vec![]));
        module.add_type(start_function_type);

        let start_function = Function::new(
            1,
            ResultType::new(vec![]),
            Expression::new(vec![Instruction::Control(ControlInstruction::Nop)]),
        );
        module.add_function(start_function);

        let element = Element::new(
            ReferenceType::Function,
            ElementMode::Passive,
            ElementInitializer::FunctionIndex(vec![0]),
        );
        module.add_element(element);

        let data = Data::new(DataMode::Passive, vec![42]);
        module.add_data(data);

        let table = Table::new(TableType::new(Limit::new(1, None), ReferenceType::Function));
        module.add_table(table);

        let memory = Memory::new(MemoryType::new(Limit::new(1, None)));
        module.add_memory(memory);

        let export = Export::new(
            Name::new("foobar".to_string()),
            ExportDescription::Function(0),
        );
        module.add_export(export);

        let start = Start::new(1);
        module.set_start(Some(start));

        let global = Global::new(
            GlobalType::new(false, ValueType::Number(NumberType::I64)),
            Expression::new(vec![Instruction::Numeric(NumericInstruction::I64Constant(
                0,
            ))]),
        );
        module.add_global(global);

        let result = SyntaxCheck::validate(module);

        assert!(result.is_ok());
    }

    #[test]
    fn valid_module_type_only() {
        let mut module = web_assembly::Module::new();
        let function_type = FunctionType::new(
            ResultType::new(vec![ValueType::Number(NumberType::I64)]),
            ResultType::new(vec![ValueType::Number(NumberType::F64)]),
        );
        module.add_type(function_type);

        let result = SyntaxCheck::validate(module);

        assert!(result.is_ok());
    }

    #[test]
    fn valid_module_function() {
        let mut module = web_assembly::Module::new();
        let function_type = FunctionType::new(
            ResultType::new(vec![ValueType::Number(NumberType::I64)]),
            ResultType::new(vec![ValueType::Number(NumberType::F64)]),
        );
        module.add_type(function_type);

        let function = Function::new(
            0,
            ResultType::new(vec![ValueType::Number(NumberType::I32)]),
            Expression::new(vec![Instruction::Numeric(NumericInstruction::F64Constant(
                0.0,
            ))]),
        );
        module.add_function(function);

        SyntaxCheck::validate(module).unwrap();
    }

    #[test]
    fn valid_module_start() {
        let mut module = web_assembly::Module::new();
        let function_type = FunctionType::new(ResultType::new(vec![]), ResultType::new(vec![]));
        module.add_type(function_type);

        let function = Function::new(
            0,
            ResultType::new(vec![]),
            Expression::new(vec![Instruction::Control(ControlInstruction::Nop)]),
        );
        module.add_function(function);

        let start = Start::new(0);
        module.set_start(Some(start));

        SyntaxCheck::validate(module).unwrap();
    }

    #[test]
    fn valid_module_element() {
        let mut module = web_assembly::Module::new();

        let function_type = FunctionType::new(
            ResultType::new(vec![ValueType::Number(NumberType::I64)]),
            ResultType::new(vec![ValueType::Number(NumberType::F64)]),
        );
        module.add_type(function_type);

        let function = Function::new(
            0,
            ResultType::new(vec![ValueType::Number(NumberType::I32)]),
            Expression::new(vec![Instruction::Numeric(NumericInstruction::F64Constant(
                0.0,
            ))]),
        );
        module.add_function(function);

        let element = Element::new(
            ReferenceType::Function,
            ElementMode::Passive,
            ElementInitializer::FunctionIndex(vec![0]),
        );
        module.add_element(element);

        let table = Table::new(TableType::new(Limit::new(0, None), ReferenceType::Function));
        module.add_table(table);

        SyntaxCheck::validate(module).unwrap();
    }

    #[test]
    fn valid_module_table_only() {
        let mut module = web_assembly::Module::new();

        let table = Table::new(TableType::new(Limit::new(0, None), ReferenceType::Function));
        module.add_table(table);

        SyntaxCheck::validate(module).unwrap();
    }

    #[test]
    fn valid_module_data() {
        let mut module = web_assembly::Module::new();

        let data = Data::new(DataMode::Passive, vec![1]);
        module.add_data(data);

        let memory = Memory::new(MemoryType::new(Limit::new(0, None)));
        module.add_memory(memory);

        SyntaxCheck::validate(module).unwrap();
    }

    #[test]
    fn valid_module_memory_only() {
        let mut module = web_assembly::Module::new();

        let memory = Memory::new(MemoryType::new(Limit::new(0, None)));
        module.add_memory(memory);

        SyntaxCheck::validate(module).unwrap();
    }

    #[test]
    fn valid_module_global_only() {
        let mut module = web_assembly::Module::new();

        let global = Global::new(
            GlobalType::new(false, ValueType::Number(NumberType::I64)),
            Expression::new(vec![Instruction::Numeric(NumericInstruction::I64Constant(
                0,
            ))]),
        );
        module.add_global(global);

        SyntaxCheck::validate(module).unwrap();
    }

    #[test]
    fn valid_module_import_only() {
        let mut module = web_assembly::Module::new();

        let export = Export::new(
            Name::new("foobar".to_string()),
            ExportDescription::Global(0),
        );
        module.add_export(export);

        let global = Global::new(
            GlobalType::new(false, ValueType::Number(NumberType::I64)),
            Expression::new(vec![Instruction::Numeric(NumericInstruction::I64Constant(
                0,
            ))]),
        );
        module.add_global(global);

        SyntaxCheck::validate(module).unwrap();
    }

    fn invalid_module() {
        let mut module = web_assembly::Module::new();

        // function with no corresponding type.
        let function = Function::new(
            0,
            ResultType::new(vec![ValueType::Number(NumberType::I32)]),
            Expression::new(vec![Instruction::Control(ControlInstruction::Nop)]),
        );
        module.add_function(function);

        let result = SyntaxCheck::validate(module);

        assert!(result.is_err());
    }
}
