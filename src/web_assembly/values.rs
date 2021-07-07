/// Names are strings denoting a literal character sequence.
/// A name string must form a valid UTF-8 encoding as defined by Unicode (Section 2.5)
/// and is interpreted as a string of Unicode scalar values.
pub struct Name(String);

impl Name {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_bytes()
    }
}
