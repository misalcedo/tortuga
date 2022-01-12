use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Program;

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}