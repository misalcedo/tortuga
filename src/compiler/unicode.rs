//! Character extensions to query for Unicode properties useful for the Tortuga compiler.

/// Character extension to test for Unicode properties.
///
/// See <https://unicode.org/reports/tr44/#Properties>
pub trait UnicodeProperties {
    /// Tests whether this is a Unicode character with the `Pattern_White_space` property.
    ///
    /// See <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3APattern_White_Space%3A%5D&g=&i=>
    fn is_pattern_white_space(&self) -> bool;
}

impl UnicodeProperties for char {
    fn is_pattern_white_space(&self) -> bool {
        match self {
            '\u{0009}'
            | '\u{000A}'..='\u{000D}'
            | '\u{0020}'
            | '\u{0085}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{2028}'
            | '\u{2029}' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_white_space() {
        assert!('\u{000B}'.is_pattern_white_space());
        assert!(!'a'.is_pattern_white_space());
    }
}
