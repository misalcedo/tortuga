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
