use crate::compiler::CompilerError;
use crate::syntax::tortuga::Process;
use crate::syntax::web_assembly::{
    Export, ExportDescription, Limit, Memory, MemoryIndex, MemoryType, Module, Name,
};

#[tracing::instrument]
pub async fn transform(process: &Process) -> Result<Module, CompilerError> {
    let mut module = Module::new();

    define_memory(&mut module)?;

    tracing::trace!(
        "Transformed process {:?} into a module.",
        process.identifier.path
    );

    Ok(module)
}

fn define_memory(module: &mut Module) -> Result<MemoryIndex, CompilerError> {
    let limit = Limit::new(1, None);
    let memory_type = MemoryType::new(limit);
    let memory = Memory::new(memory_type);
    let memory_index = module.add_memory(memory);

    let name = Name::new("io".to_string());
    let export_description = ExportDescription::Memory(memory_index);
    let export = Export::new(name, export_description);

    module.add_export(export);

    Ok(memory_index)
}
