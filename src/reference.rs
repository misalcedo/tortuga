use serde::export::Formatter;
use std::fmt::{Display, Result};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reference {
    global: Uuid,
}

impl Reference {
    pub(crate) fn new() -> Reference {
        Reference {
            global: Uuid::new_v4(),
        }
    }

    pub fn from(hyphenated: &str) -> Reference {
        Reference {
            global: Uuid::parse_str(hyphenated).unwrap(),
        }
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.global.to_hyphenated())
    }
}
