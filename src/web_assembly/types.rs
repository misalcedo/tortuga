#[derive(Copy, Clone)]
pub enum NumberType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Copy, Clone)]
pub enum IntegerType {
    I32,
    I64,
}

#[derive(Copy, Clone)]
pub enum FloatType {
    F32,
    F64,
}

#[derive(Copy, Clone)]
pub enum ReferenceType {
    Function, // funcref
    External, // externref
}

#[derive(Copy, Clone)]
pub enum HeapType {
    Function, // func
    External, // extern
}

#[derive(Copy, Clone)]
pub enum ValueType {
    Number(NumberType),
    Reference(ReferenceType),
}

#[derive(Clone)]
pub struct ResultType {
    types: Vec<ValueType>,
}

impl ResultType {
    pub fn new(types: Vec<ValueType>) -> Self {
        ResultType { types }
    }

    pub fn value_types(&self) -> &[ValueType] {
        &self.types
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
}

#[derive(Clone)]
pub struct FunctionType {
    parameters: ResultType,
    results: ResultType,
}

impl FunctionType {
    pub fn new(parameters: ResultType, results: ResultType) -> Self {
        FunctionType {
            parameters,
            results,
        }
    }

    pub fn parameters(&self) -> &ResultType {
        &self.parameters
    }

    pub fn results(&self) -> &ResultType {
        &self.results
    }
}

#[derive(Copy, Clone)]
pub struct Limit {
    min: usize,
    max: Option<usize>,
}

impl Limit {
    pub fn new(min: usize, max: Option<usize>) -> Self {
        Limit { min, max }
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }
}

#[derive(Copy, Clone)]
pub struct MemoryType {
    limits: Limit,
}

impl MemoryType {
    pub fn new(limits: Limit) -> Self {
        MemoryType { limits }
    }

    pub fn limits(&self) -> &Limit {
        &self.limits
    }
}

#[derive(Copy, Clone)]
pub struct TableType {
    limits: Limit,
    kind: ReferenceType,
}

impl TableType {
    pub fn new(limits: Limit, reference_type: ReferenceType) -> Self {
        TableType {
            limits,
            kind: reference_type,
        }
    }

    pub fn limits(&self) -> &Limit {
        &self.limits
    }

    pub fn kind(&self) -> &ReferenceType {
        &self.kind
    }
}

#[derive(Copy, Clone)]
pub struct GlobalType {
    is_mutable: bool,
    kind: ValueType,
}

impl GlobalType {
    pub fn new(is_mutable: bool, kind: ValueType) -> Self {
        GlobalType { is_mutable, kind }
    }

    pub fn kind(&self) -> &ValueType {
        &self.kind
    }

    pub fn is_mutable(&self) -> bool {
        self.is_mutable
    }
}

#[derive(Clone)]
pub enum ExternalType {
    Function(FunctionType),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}
