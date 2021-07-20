mod instruction;
mod module;
mod types;
mod values;

use crate::compiler::emitter::module::emit_module;
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::Module;
use std::io::Write;
pub use types::*;
pub use values::*;

/// Emits a binary representation of a WebAssembly Abstract Syntax Tree (AST) to a `Write` output.
pub async fn emit_binary<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_module(&module, output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::about;
    use crate::syntax::web_assembly::{
        self, ControlInstruction, Data, DataMode, Element, ElementInitializer, ElementMode, Export,
        ExportDescription, Expression, Function, FunctionType, Global, GlobalType, Import,
        ImportDescription, Instruction, Limit, Memory, MemoryType, Name, NumberType,
        NumericInstruction, ReferenceType, ResultType, Start, Table, TableType, ValueType,
    };
    use wasmtime::{Engine, Extern, Func, Instance, Module, Store};

    fn validate(target: &web_assembly::Module) -> Result<(), CompilerError> {
        let mut bytes = Vec::new();

        emit_module(&target, &mut bytes)?;

        let engine = Engine::default();
        let module = Module::new(&engine, &bytes)?;
        let mut store = Store::new(&engine, 0);
        let mut imports: Vec<Extern> = Vec::new();

        if !target.imports().is_empty() {
            let start = Func::wrap(&mut store, || {});
            imports.push(start.into());
        }

        Instance::new(&mut store, &module, &imports)?;

        Ok(())
    }

    #[test]
    fn empty_module() {
        let mut buffer = Vec::new();
        let module = web_assembly::Module::new();
        let result = validate(&module);

        assert!(result.is_ok());

        emit_module(&module, &mut buffer).unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        let prefix = b"\x00\x61\x73\x6D\x01\x00\x00\x00";
        let section = b"\x00";
        let name = "version".as_bytes();
        let version = about::VERSION.as_bytes();
        let size = name.len() + version.len() + 1;

        bytes.extend(prefix);
        bytes.extend(section);
        bytes.push(size as u8);
        bytes.push(name.len() as u8);
        bytes.extend(name);
        bytes.extend(version);

        assert_eq!(&buffer, &bytes);
    }

    #[test]
    fn valid_empty_module() {
        let module = web_assembly::Module::new();
        let result = validate(&module);

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

        let import = Import::new(
            Name::new("test".to_string()),
            Name::new("foobar".to_string()),
            ImportDescription::Function(1),
        );
        module.add_import(import);

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

        let start = Start::new(0);
        module.set_start(Some(start));

        let global = Global::new(
            GlobalType::new(false, ValueType::Number(NumberType::I64)),
            Expression::new(vec![Instruction::Numeric(NumericInstruction::I64Constant(
                0,
            ))]),
        );
        module.add_global(global);

        let result = validate(&module);

        assert!(result.is_ok());
    }

    #[test]
    fn valid_module_import() {
        let mut module = web_assembly::Module::new();

        let start_function_type =
            FunctionType::new(ResultType::new(vec![]), ResultType::new(vec![]));
        module.add_type(start_function_type);

        let import = Import::new(
            Name::new("test".to_string()),
            Name::new("foobar".to_string()),
            ImportDescription::Function(0),
        );
        module.add_import(import);

        let result = validate(&module);

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

        let result = validate(&module);

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

        validate(&module).unwrap();
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

        validate(&module).unwrap();
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

        validate(&module).unwrap();
    }

    #[test]
    fn valid_module_table_only() {
        let mut module = web_assembly::Module::new();

        let table = Table::new(TableType::new(Limit::new(0, None), ReferenceType::Function));
        module.add_table(table);

        validate(&module).unwrap();
    }

    #[test]
    fn valid_module_data() {
        let mut module = web_assembly::Module::new();

        let data = Data::new(DataMode::Passive, vec![1]);
        module.add_data(data);

        let memory = Memory::new(MemoryType::new(Limit::new(0, None)));
        module.add_memory(memory);

        validate(&module).unwrap();
    }

    #[test]
    fn valid_module_memory_only() {
        let mut module = web_assembly::Module::new();

        let memory = Memory::new(MemoryType::new(Limit::new(0, None)));
        module.add_memory(memory);

        validate(&module).unwrap();
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

        validate(&module).unwrap();
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

        validate(&module).unwrap();
    }

    #[test]
    fn invalid_module() {
        let mut module = web_assembly::Module::new();

        // function with no corresponding type.
        let function = Function::new(
            0,
            ResultType::new(vec![ValueType::Number(NumberType::I32)]),
            Expression::new(vec![Instruction::Control(ControlInstruction::Nop)]),
        );
        module.add_function(function);

        let result = validate(&module);

        assert!(result.is_err());
    }
}
