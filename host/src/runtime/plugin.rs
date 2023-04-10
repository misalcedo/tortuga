use crate::runtime::{Identifier, Uri};

#[derive(Clone, Debug)]
pub struct Plugin {
    identifier: Identifier,
    uri: Uri,
}

impl AsRef<Identifier> for Plugin {
    fn as_ref(&self) -> &Identifier {
        &self.identifier
    }
}

impl Plugin {
    pub fn new(uri: String) -> Self {
        Plugin {
            identifier: Identifier::from(uri.as_str()),
            uri: Uri::from(uri),
        }
    }

    pub fn identifier(&self) -> Identifier {
        self.identifier
    }
}
