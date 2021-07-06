use crate::web_assembly;

/// Names are strings denoting a literal character sequence.
/// A name string must form a valid UTF-8 encoding as defined by Unicode (Section 2.5)
/// and is interpreted as a string of Unicode scalar values.
pub struct Name(web_assembly::String);
