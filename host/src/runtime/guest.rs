use crate::runtime::Identifier;
use crate::runtime::Uri;

#[derive(Clone, Debug)]
pub struct Guest {
    identifier: Identifier,
    uri: Uri,
}

impl AsRef<Identifier> for Guest {
    fn as_ref(&self) -> &Identifier {
        &self.identifier
    }
}

impl Guest {
    pub fn new(uri: String) -> Self {
        Guest {
            identifier: Identifier::from(uri.as_str()),
            uri: Uri::from(uri),
        }
    }

    pub fn identifier(&self) -> Identifier {
        self.identifier
    }
}
