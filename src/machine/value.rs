#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    #[default]
    Number,
    Closure,
    Identifier
}