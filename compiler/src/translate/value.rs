use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Any,
    Uninitialized(usize),
    Closure,
    Number(Option<usize>),
    Text(Option<usize>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Any => write!(f, "{:?}", self),
            Value::Uninitialized(_) => write!(f, "{:?}", self),
            Value::Closure => write!(f, "{:?}", self),
            Value::Number(Some(o)) => write!(f, "ConstantNumber({})", o),
            Value::Number(None) => write!(f, "Number"),
            Value::Text(Some(o)) => write!(f, "ConstantText({})", o),
            Value::Text(None) => write!(f, "Text"),
        }
    }
}
