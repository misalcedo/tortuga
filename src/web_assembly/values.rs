/// Names are strings denoting a literal character sequence.
/// A name string must form a valid UTF-8 encoding as defined by Unicode (Section 2.5)
/// and is interpreted as a string of Unicode scalar values.
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Name(name)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_bytes()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct Bytes<'a>(&'a [u8]);

impl<'a> Bytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Bytes::<'a>(bytes)
    }

    pub fn as_ref(&self) -> &[u8] {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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
