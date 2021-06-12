pub struct Module {
    pub name: Option<Identifier>,
    pub types: Vec<Type>,
    functions: Vec<Box<dyn Function>>,
    tables: Vec<Box<dyn Table>>,
    memories: Vec<Box<dyn Memory>>,
    globals: Vec<Box<dyn Global>>,
    elements: Vec<Box<Element>>,
    data: Vec<Data>,
    start: Option<Start>
}

impl Module {
    /// Instantiates an empty module with the given name.
    pub fn new(name: Option<&str>) -> Module {
        Module {
            name: name.map(Identifier::new),
            types: vec![],
            functions: vec![],
            tables: vec![],
            memories: vec![],
            globals: vec![],
            elements: vec![],
            data: vec![],
            start: None
        }
    }
}

trait Function {}
trait Table {}
trait Memory {}
trait Global {}

pub struct Identifier {
    pub value: String
}

impl Identifier {
    pub fn new(value: &str) -> Identifier {
        Identifier {
            value: String::from(value)
        }
    }
}

pub struct Type {
    pub name: Option<Identifier>,
    pub function_type: FunctionType
}

impl Type {
    pub fn new(name: Option<&str>, function_type: FunctionType) -> Type {
        Type {
            name: name.map(Identifier::new),
            function_type
        }
    }
}

pub struct FunctionType {
    pub parameters: Vec<Param>,
    pub results: Vec<Result>
}

impl FunctionType {
    pub fn new(parameters: Vec<Param>, results: Vec<Result>) -> FunctionType {
        FunctionType {
            parameters,
            results
        }
    }
}

pub struct Param {
    name: Option<Identifier>,
    value_type: ValueType
}

impl Param {
    pub fn new(name: Option<&str>, value_type: ValueType) -> Param {
        Param {
            name: name.map(Identifier::new),
            value_type
        }
    }
}

pub struct Result {
    pub value_type: ValueType
}

impl Result {
    pub fn new(value_type: ValueType) -> Result {
        Result {
            value_type
        }
    }
}

pub enum ValueType {
    I32,
    I64,
    F32,
    F64
}

struct Import;
struct Export;
struct Start;
struct Element;
struct Data;