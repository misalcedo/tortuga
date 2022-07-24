use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}