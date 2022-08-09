use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Code(Vec<u8>);

impl From<&[u8]> for Code {
    fn from(code: &[u8]) -> Self {
        Code(code.to_vec())
    }
}

impl From<Vec<u8>> for Code {
    fn from(code: Vec<u8>) -> Self {
        Code(code)
    }
}

impl From<Code> for Vec<u8> {
    fn from(code: Code) -> Self {
        code.0
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Code {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
}
