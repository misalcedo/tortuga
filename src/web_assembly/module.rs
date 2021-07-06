pub struct Module {
    types: Vec<Type>,
    imports: Vec<Import>,
    functions: Vec<Function>,
    tables: Vec<Table>,
    memories: Vec<Memory>,
    globals: Vec<Global>,
    exports: Vec<Export>,
    start: Start,
    elements: Vec<Elements>,
    data: Vec<Data>,
}
