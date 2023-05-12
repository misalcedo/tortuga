pub use name::Name;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use value::Value;

mod name;
mod value;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Headers(HashMap<Name, Value>);

impl Headers {
    pub fn set<V>(&mut self, name: Name, value: V) -> Option<Value>
    where
        V: Into<Value>,
    {
        self.0.insert(name, value.into())
    }

    pub fn get(&mut self, name: &Name) -> Option<&Value> {
        self.0.get(name)
    }
}
