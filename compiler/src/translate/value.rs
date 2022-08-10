#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Any,
    Unknown,
    Number(Option<usize>),
    Text(Option<usize>),
}
