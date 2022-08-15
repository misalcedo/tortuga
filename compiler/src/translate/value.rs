use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Any,
    Uninitialized(usize),
    Closure,
    Grouping(Vec<Value>),
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
            Value::Grouping(parts) => {
                write!(f, "(")?;

                let mut iterator = parts.iter().peekable();

                while let Some(next) = iterator.next() {
                    write!(f, "{}", next)?;

                    if iterator.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }

                write!(f, ")")
            }
        }
    }
}
