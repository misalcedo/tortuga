use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Value {
    Single(String),
    Multiple(Vec<String>),
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::Single(value.to_string())
    }
}

impl<'a> From<String> for Value {
    fn from(value: String) -> Self {
        Value::Single(value)
    }
}
