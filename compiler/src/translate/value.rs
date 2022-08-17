use std::fmt::{Display, Formatter};
use std::mem;

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, Hash)]
pub enum Value {
    #[default]
    Any,
    None,
    Uninitialized(usize),
    UninitializedFunction(usize, Box<Value>),
    Closure(usize),
    Boolean,
    Group(Vec<Value>),
    Number(Option<usize>),
    Text(Option<usize>),
    Function(Box<Value>, Box<Value>),
}

impl Value {
    pub fn group(mut group: Vec<Value>) -> Self {
        if group.is_empty() {
            Value::None
        } else if group.len() == 1 {
            group.pop().unwrap_or_default()
        } else {
            Value::Group(group)
        }
    }

    pub fn function(parameters: Value, results: Value) -> Self {
        Value::Function(parameters.into(), results.into())
    }

    pub fn uninitialized_function(local: usize, parameters: Value) -> Self {
        Value::UninitializedFunction(local, parameters.into())
    }

    pub fn len(&self) -> usize {
        match self {
            Value::Group(a) => a.len(),
            Value::Function(a, _) => a.len(),
            _ => 1,
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        match self {
            Value::Group(a) => Iter::Group(a.iter()),
            Value::UninitializedFunction(_, a) => a.iter(),
            Value::Function(a, _) => a.iter(),
            _ => Iter::Singleton(Some(self)),
        }
    }
}

pub enum Iter<'a> {
    Singleton(Option<&'a Value>),
    Group(std::slice::Iter<'a, Value>),
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Singleton(a) => a.take(),
            Iter::Group(a) => a.next(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => true,
            (Self::UninitializedFunction(_, _), _) => self.iter().eq(other.iter()),
            (_, Self::UninitializedFunction(_, _)) => self.iter().eq(other.iter()),
            (Self::Group(_), _) => self.iter().eq(other.iter()),
            (_, Self::Group(_)) => self.iter().eq(other.iter()),
            (Self::Function(ap, ar), Self::Function(bp, br)) => ap == bp && ar == br,
            (Self::Function(_, _), _) => self.iter().eq(other.iter()),
            (_, Self::Function(_, _)) => self.iter().eq(other.iter()),
            (a, b) => mem::discriminant(a) == mem::discriminant(b),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Any => write!(f, "{:?}", self),
            Value::None => write!(f, "{:?}", self),
            Value::Uninitialized(_) => write!(f, "{:?}", self),
            Value::UninitializedFunction(_, _) => write!(f, "{:?}", self),
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
                write!(f, "Function({} => {})", parameters, results)
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
        assert_eq!(Value::group(vec![Value::Boolean]), Value::Boolean);
        assert_eq!(Value::Boolean, Value::group(vec![Value::Boolean]));
        assert_eq!(
            Value::group(vec![Value::Boolean]),
            Value::group(vec![Value::Boolean])
        );
        assert_ne!(
            Value::group(vec![Value::Boolean, Value::Boolean]),
            Value::group(vec![Value::Boolean])
        );
        assert_ne!(
            Value::group(vec![Value::Boolean]),
            Value::group(vec![Value::Text(None)])
        );
    }

    #[test]
    fn functions() {
        assert_eq!(
            Value::function(Value::Boolean, Value::None),
            Value::group(vec![Value::Boolean])
        );
        assert_eq!(
            Value::function(Value::Boolean, Value::None),
            Value::function(Value::group(vec![Value::Boolean]), Value::None)
        );
        assert_eq!(
            Value::group(vec![Value::group(vec![Value::Boolean])]),
            Value::function(Value::group(vec![Value::Boolean]), Value::group(vec![]))
        );
        assert_ne!(
            Value::function(
                Value::group(vec![Value::Boolean]),
                Value::group(vec![Value::Closure(0)])
            ),
            Value::function(Value::group(vec![Value::Boolean]), Value::group(vec![]))
        );
    }

    #[test]
    fn closure() {
        assert_eq!(Value::Closure(0), Value::Closure(1));
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
