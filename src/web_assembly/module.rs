use crate::web_assembly::{
    Data, Element, Export, Function, Global, Import, Memory, Start, Table, Type,
};

pub struct Module {
    types: Vec<Type>,
    imports: Vec<Import>,
    functions: Vec<Function>,
    tables: Vec<Table>,
    memories: Vec<Memory>,
    globals: Vec<Global>,
    exports: Vec<Export>,
    start: Start,
    elements: Vec<Element>,
    data: Vec<Data>,
}
