use std::fmt::{Display, Formatter};
use std::mem;

#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub enum Value {
    Any,
    Uninitialized(usize),
    Closure(Option<usize>),
    Boolean,
    Group(Vec<Value>),
    Number(Option<usize>),
    Text(Option<usize>),
    Function(Vec<Value>, Vec<Value>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => true,
            (Self::Function(a, _), Self::Group(b)) if a.len() == b.len() => a == b,
            (Self::Group(a), Self::Function(b, _)) if a.len() == b.len() => a == b,
            (Self::Group(a), b) if a.len() == 1 => &a[0] == b,
            (a, Self::Group(b)) if b.len() == 1 => a == &b[0],
            (Self::Function(aa, ar), Self::Function(ba, br)) => aa == ba && ar == br,
            (Self::Group(a), Self::Group(b)) => a.iter().eq(b),
            (a, b) => mem::discriminant(a) == mem::discriminant(b),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Any => write!(f, "{:?}", self),
            Value::Uninitialized(_) => write!(f, "{:?}", self),
            Value::Closure(_) => write!(f, "{:?}", self),
            Value::Boolean => write!(f, "{:?}", self),
            Value::Number(Some(o)) => write!(f, "ConstantNumber({})", o),
            Value::Number(None) => write!(f, "Number"),
            Value::Text(Some(o)) => write!(f, "ConstantText({})", o),
            Value::Text(None) => write!(f, "Text"),
            Value::Group(parts) => {
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
            Value::Function(parameters, results) => {
                write!(f, "Function({} => {})", parameters.len(), results.len())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any() {
        assert_eq!(Value::Any, Value::Boolean);
        assert_eq!(Value::Boolean, Value::Any);
    }

    #[test]
    fn groups() {
        assert_eq!(Value::Group(vec![Value::Boolean]), Value::Boolean);
        assert_eq!(Value::Boolean, Value::Group(vec![Value::Boolean]));
        assert_eq!(
            Value::Group(vec![Value::Boolean]),
            Value::Group(vec![Value::Boolean])
        );
        assert_ne!(
            Value::Group(vec![Value::Boolean, Value::Boolean]),
            Value::Group(vec![Value::Boolean])
        );
        assert_ne!(
            Value::Group(vec![Value::Boolean]),
            Value::Group(vec![Value::Text(None)])
        );
    }

    #[test]
    fn functions() {
        assert_eq!(
            Value::Function(vec![Value::Boolean], vec![]),
            Value::Group(vec![Value::Boolean])
        );
        assert_eq!(
            Value::Function(vec![Value::Boolean], vec![]),
            Value::Function(vec![Value::Boolean], vec![])
        );
        assert_eq!(
            Value::Group(vec![Value::Group(vec![Value::Boolean])]),
            Value::Function(vec![Value::Boolean], vec![])
        );
        assert_ne!(
            Value::Function(vec![Value::Boolean], vec![Value::Closure(None)]),
            Value::Function(vec![Value::Boolean], vec![])
        );
    }

    #[test]
    fn closure() {
        assert_eq!(Value::Closure(None), Value::Closure(Some(1)));
    }

    #[test]
    fn number() {
        assert_eq!(Value::Number(None), Value::Number(Some(1)));
    }

    #[test]
    fn text() {
        assert_eq!(Value::Text(None), Value::Text(Some(1)));
    }
}
