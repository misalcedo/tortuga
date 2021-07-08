#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IntegerType {
    I32,
    I64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FloatType {
    F32,
    F64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReferenceType {
    Function, // funcref
    External, // externref
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HeapType {
    Function, // func
    External, // extern
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    Number(NumberType),
    Reference(ReferenceType),
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExternalType {
    Function(FunctionType),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_function_type() {
        let result_type = ResultType::new(Vec::new());
        let function_type = FunctionType::new(result_type.clone(), result_type.clone());

        assert!(function_type.parameters().is_empty());
        assert!(function_type.results().is_empty());
    }

    #[test]
    fn new_result_type() {
        let result_type = ResultType::new(vec![
            ValueType::Number(NumberType::I64),
            ValueType::Number(NumberType::F64),
        ]);

        assert_eq!(result_type.len(), 2);
        assert!(!result_type.is_empty());
        assert_eq!(
            result_type.value_types(),
            &[
                ValueType::Number(NumberType::I64),
                ValueType::Number(NumberType::F64),
            ]
        );
    }

    #[test]
    fn new_limit() {
        let max = Some(2);
        let min = 0;
        let limit = Limit::new(min, max);

        assert_eq!(limit.min, min);
        assert_eq!(limit.max, max);
    }

    #[test]
    fn new_memory_type() {
        let limit = Limit::new(0, None);
        let memory_type = MemoryType::new(limit.clone());

        assert_eq!(memory_type.limits(), &limit);
    }

    #[test]
    fn new_table_type() {
        let limit = Limit::new(0, None);
        let reference_type = ReferenceType::External;
        let table_type = TableType::new(limit.clone(), reference_type);

        assert_eq!(table_type.limits(), &limit);
        assert_eq!(table_type.kind(), &reference_type);
    }

    #[test]
    fn new_global_type() {
        let is_mutable = true;
        let kind = ValueType::Number(NumberType::I64);
        let global_type = GlobalType::new(is_mutable, kind);

        assert_eq!(global_type.is_mutable(), is_mutable);
        assert_eq!(global_type.kind(), &kind);
    }
}
