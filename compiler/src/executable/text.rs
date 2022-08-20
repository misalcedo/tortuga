use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Text(String);

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Text(text.to_string())
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Text(text)
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Text {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
