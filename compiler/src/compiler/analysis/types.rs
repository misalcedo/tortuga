use std::fmt::{Display, Formatter};
use std::mem;

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, Hash)]
pub enum Type {
    #[default]
    None,
    Error,
    Boolean,
    Group(Vec<Type>),
    Number(Option<usize>),
    Text(Option<usize>),
    Function(Box<Type>, Box<Type>, Box<Type>),
    Reference(ReferenceKind, usize),
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReferenceKind {
    Local,
    Capture,
    Function,
}

impl Type {
    pub fn group(mut group: Vec<Type>) -> Self {
        if group.is_empty() {
            Type::None
        } else if group.len() == 1 {
            group.pop().unwrap_or_default()
        } else {
            Type::Group(group)
        }
    }

    pub fn function(parameters: Type, captures: Type, results: Type) -> Self {
        Type::Function(parameters.into(), captures.into(), results.into())
    }

    pub fn converts_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Error, _) | (_, Self::Error) => true,
            (Self::Function(ap, ac, ar), Self::Function(bp, bc, br)) => {
                ap.converts_to(bp) && ac.converts_to(bc) && ar.converts_to(br)
            }
            (_, Self::Function(parameters, _, _)) => self.iter().eq(parameters.iter()),
            (Self::Group(_), _) | (_, Self::Group(_)) => self.iter().eq(other.iter()),
            (a, b) => a == b,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Type::Group(a) => a.len(),
            Type::Function(a, _, _) => a.len(),
            _ => 1,
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        match self {
            Type::Group(a) => Iter::Group(a.iter()),
            Type::Function(a, _, _) => a.iter(),
            _ => Iter::Singleton(Some(self)),
        }
    }
}

pub enum Iter<'a> {
    Singleton(Option<&'a Type>),
    Group(std::slice::Iter<'a, Type>),
    Function(Box<Self>, Box<Self>, Box<Self>),
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Singleton(a) => a.take(),
            Iter::Group(a) => a.next(),
            Iter::Function(a, b, c) => a.next().or_else(|| b.next()).or_else(|| c.next()),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number(Some(o)) => write!(f, "ConstantNumber({})", o),
            Type::Number(None) => write!(f, "Number"),
            Type::Text(Some(o)) => write!(f, "ConstantText({})", o),
            Type::Text(None) => write!(f, "Text"),
            Type::Group(parts) => {
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
            Type::Function(parameters, _, results) => {
                write!(f, "Function({} => {})", parameters, results)
            }
            _ => write!(f, "{:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error() {
        assert!(Type::Error.converts_to(&Type::Boolean));
        assert!(Type::Boolean.converts_to(&Type::Error));
        assert_eq!(Type::Error, Type::Error);
        assert_ne!(Type::Error, Type::Boolean);
    }

    #[test]
    fn groups() {
        assert_eq!(Type::group(vec![Type::Boolean]), Type::Boolean);
        assert!(Type::Boolean.converts_to(&Type::group(vec![Type::Boolean])));
        assert_eq!(
            Type::group(vec![Type::Boolean]),
            Type::group(vec![Type::Boolean])
        );
        assert_ne!(
            Type::group(vec![Type::Boolean, Type::Boolean]),
            Type::group(vec![Type::Boolean])
        );
        assert_ne!(
            Type::group(vec![Type::Boolean]),
            Type::group(vec![Type::Text(None)])
        );
    }

    #[test]
    fn functions() {
        assert!(
            Type::group(vec![Type::Boolean]).converts_to(&Type::function(
                Type::Boolean,
                Type::None,
                Type::None
            ))
        );
        assert!(!Type::function(Type::Boolean, Type::None, Type::None)
            .converts_to(&Type::group(vec![Type::Boolean])));
        assert_eq!(
            Type::function(Type::Boolean, Type::None, Type::None),
            Type::function(Type::group(vec![Type::Boolean]), Type::None, Type::None)
        );
        assert!(
            Type::group(vec![Type::Boolean]).converts_to(&Type::function(
                Type::Boolean,
                Type::None,
                Type::Boolean
            ))
        );
        assert_eq!(
            Type::function(
                Type::group(vec![Type::Boolean]),
                Type::None,
                Type::group(vec![Type::Reference(ReferenceKind::Function, 0)])
            ),
            Type::function(
                Type::group(vec![Type::Boolean]),
                Type::None,
                Type::group(vec![])
            )
        );
        assert!(!Type::function(
            Type::group(vec![Type::Boolean]),
            Type::None,
            Type::group(vec![Type::Reference(ReferenceKind::Function, 0)])
        )
        .converts_to(&Type::function(
            Type::group(vec![Type::Boolean]),
            Type::None,
            Type::group(vec![])
        )));
    }

    #[test]
    fn closure() {
        assert_eq!(
            Type::Reference(ReferenceKind::Function, 0),
            Type::Reference(ReferenceKind::Function, 1)
        );
    }

    #[test]
    fn number() {
        assert_eq!(Type::Number(None), Type::Number(Some(1)));
        assert_eq!(Type::Number(None), Type::Number(Some(1)));
    }

    #[test]
    fn text() {
        assert_eq!(Type::Text(None), Type::Text(Some(1)));
    }
}
