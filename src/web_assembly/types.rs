pub enum NumberType {
    I32,
    I64,
    F32,
    F64,
}

pub enum IntegerType {
    I32,
    I64,
}

pub enum FloatType {
    F32,
    F64,
}

pub enum ReferenceType {
    Function, // funcref
    External, // externref
}

pub enum HeapType {
    Function, // func
    External, // extern
}

pub enum ValueType {
    Number(NumberType),
    Reference(ReferenceType),
}
