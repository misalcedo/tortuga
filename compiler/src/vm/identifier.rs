use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier(u128);

impl From<u128> for Identifier {
    fn from(identifier: u128) -> Self {
        Identifier(identifier)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
