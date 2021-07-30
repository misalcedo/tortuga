use crate::compiler::CompilerError;
use crate::syntax::tortuga::Process;
use crate::syntax::web_assembly::{
    Data, DataMode, Element, ElementInitializer, ElementMode, Export, ExportDescription,
    Expression, Function, FunctionType, Global, GlobalType, Import, ImportDescription, Instruction, Limit, Memory, MemoryType,
    Module, Name, NumberType, NumericInstruction, ReferenceType, ResultType, Start, Table,
    TableType, ValueType,
};

#[tracing::instrument]
pub async fn transform(process: &Process) -> Result<Module, CompilerError> {
    let mut module = Module::new();

    define_host_imports(process, &mut module)?;

    tracing::trace!("Transformed process {:?} into a module.", process.identifier.path);

    Ok(module)
}

fn define_host_imports(process: &Process, module: &mut Module) -> Result<(), CompilerError> {
    let spawn_type = FunctionType::new(ResultType::new(Vec::new()), ResultType::new(vec![ValueType::Reference(ReferenceType::External)]));
    let spawn_index = module.add_type(spawn_type);

    for child in &process.children {
        let module_name = Name::new(child.identifier.to_string());
        let function_name = Name::new("spawn".to_string());
        let description = ImportDescription::Function(spawn_index);
        let import = Import::new(module_name, function_name, description);

        module.add_import(import);
    }
    
    Ok(())
}