/// Names are strings denoting a literal character sequence.
/// A name string must form a valid UTF-8 encoding as defined by Unicode (Section 2.5)
/// and is interpreted as a string of Unicode scalar values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    value: String,
}

impl Name {
    pub fn new(name: String) -> Self {
        Name { value: name }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.value.as_bytes()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Bytes<'a> {
    value: &'a [u8],
}

impl<'a> Bytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Bytes::<'a> { value: bytes }
    }

    pub fn as_ref(&self) -> &[u8] {
        self.value
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bytes() {
        let content = [0, 1, 2];
        let bytes = Bytes::new(&content);

        assert_eq!(bytes.len(), content.len());
        assert_eq!(bytes.is_empty(), content.is_empty());
        assert_eq!(bytes.as_ref(), content);
    }

    #[test]
    fn new_name() {
        let content = "Hello, World!";
        let name = Name::new(content.to_string());

        assert_eq!(name.len(), content.len());
        assert_eq!(name.is_empty(), content.is_empty());
        assert_eq!(name.as_bytes(), content.as_bytes());
    }
}
