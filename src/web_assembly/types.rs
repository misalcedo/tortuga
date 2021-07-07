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
    pub fn value_types(&self) -> &[ValueType] {
        &self.types
    }
}

#[derive(Clone)]
pub struct FunctionType {
    parameters: ResultType,
    results: ResultType,
}

impl FunctionType {
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
    pub fn limits(&self) -> &Limit {
        &self.limits
    }
}

#[derive(Copy, Clone)]
pub struct TableType {
    limits: Limit,
    reference_type: ReferenceType,
}

impl TableType {
    pub fn limits(&self) -> &Limit {
        &self.limits
    }

    pub fn reference_type(&self) -> &ReferenceType {
        &self.reference_type
    }
}

#[derive(Copy, Clone)]
pub struct GlobalType {
    is_mutable: bool,
    value_type: ValueType,
}

impl GlobalType {
    pub fn value_type(&self) -> &ValueType {
        &self.value_type
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
