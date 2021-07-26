use crate::compiler::CompilerError;
use crate::syntax::tortuga::Process;
use crate::syntax::web_assembly::{
    Data, DataMode, Element, ElementInitializer, ElementMode, Export, ExportDescription,
    Expression, Function, FunctionType, Global, GlobalType, Instruction, Limit, Memory, MemoryType,
    Module, Name, NumberType, NumericInstruction, ReferenceType, ResultType, Start, Table,
    TableType, ValueType,
};

#[tracing::instrument]
pub async fn transform(_node: &Process) -> Result<Module, CompilerError> {
    let mut module = Module::new();
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

    let start_function_type = FunctionType::new(ResultType::new(vec![]), ResultType::new(vec![]));
    module.add_type(start_function_type);

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

    let start_function = Function::new(1, ResultType::new(vec![]), Expression::new(vec![]));
    module.add_function(start_function);
    let start = Start::new(1);
    module.set_start(Some(start));

    let global = Global::new(
        GlobalType::new(false, ValueType::Number(NumberType::I64)),
        Expression::new(vec![Instruction::Numeric(NumericInstruction::I64Constant(
            0,
        ))]),
    );
    module.add_global(global);

    tracing::trace!("Transformed a node into a module.");

    Ok(module)
}
