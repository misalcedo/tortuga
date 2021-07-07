/// Strings denote sequences of bytes that can represent both textual and binary data.
/// They are enclosed in quotation marks and may contain any character other than
/// ASCII control characters, quotation marks (‘"’), or backslash (‘∖’),
/// except when expressed with an escape sequence.
pub struct String {
    bytes: Vec<u8>,
}

/// Names are strings denoting a literal character sequence.
/// A name string must form a valid UTF-8 encoding as defined by Unicode (Section 2.5)
/// and is interpreted as a string of Unicode scalar values.
pub struct Name(self::String);

impl Name {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0.bytes
    }
}
