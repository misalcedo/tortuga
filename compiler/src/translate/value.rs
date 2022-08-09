#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Undefined,
    Number,
    ConstantNumber(usize),
    Text,
    ConstantText(usize),
}
