use crate::compiler::CompilerError;
use crate::syntax::tortuga::Process;
use crate::syntax::web_assembly::{
    Data, DataMode, Element, ElementInitializer, ElementMode, Export, ExportDescription,
    Expression, Function, FunctionIndex, FunctionType, Global, GlobalType, Import, ImportDescription, Instruction, Limit, Memory, MemoryType,
    Module, Name, NumberType, NumericInstruction, ReferenceType, ResultType, Start, Table, TableIndex,
    TableType, ValueType,
};

#[tracing::instrument]
pub async fn transform(process: &Process) -> Result<Module, CompilerError> {
    let mut module = Module::new();

    if !process.children.is_empty() {
        let spawn_indices = define_host_imports(process, &mut module)?;
        define_children_table(process, &mut module, &spawn_indices[..])?;
    }

    tracing::trace!("Transformed process {:?} into a module.", process.identifier.path);

    Ok(module)
}

fn define_children_table(process: &Process, module: &mut Module, spawn_indices: &[FunctionIndex]) -> Result<(TableIndex, TableIndex), CompilerError> {
    let spawn_limits = Limit::new(process.children.len(), Some(process.children.len()));
    let spawn_table_type = TableType::new(spawn_limits, ReferenceType::Function);
    let spawn_table = Table::new(spawn_table_type);

    let children_limits = Limit::new(process.children.len(), None);
    let children_table_type = TableType::new(children_limits, ReferenceType::External);
    let children_table = Table::new(children_table_type);

    let spawn_table_index = module.add_table(spawn_table);
    let children_table_index = module.add_table(children_table);

    let mut indices = Vec::new();
    indices.extend(spawn_indices);

    let mode = ElementMode::Active(spawn_table_index, Expression::new(Vec::new()));
    let initializers = ElementInitializer::FunctionIndex(indices);
    let spawn_elements = Element::new(ReferenceType::Function, mode, initializers);
    
    module.add_element(spawn_elements);
    
    Ok((spawn_table_index, children_table_index))
}

fn define_host_imports(process: &Process, module: &mut Module) -> Result<Vec<FunctionIndex>, CompilerError> {
    let spawn_type = FunctionType::new(ResultType::new(Vec::new()), ResultType::new(vec![ValueType::Reference(ReferenceType::External)]));
    let spawn_index = module.add_type(spawn_type);
    let mut indices = Vec::new();

    for child in &process.children {
        let module_name = Name::new(child.identifier.to_string());
        let function_name = Name::new("spawn".to_string());
        let description = ImportDescription::Function(spawn_index);
        let import = Import::new(module_name, function_name, description);

        indices.push(module.add_import(import));
    }
    
    Ok(indices)
}