use std::fmt::{Display, Formatter, Result};
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

    pub fn as_u128(&self) -> u128 {
        self.global.as_u128()
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.global.to_hyphenated())
    }
}
