//! Character extensions to query for Unicode properties useful for the Tortuga compiler.

use unicode_xid::UnicodeXID;

/// Character extension to test for Unicode properties.
///
/// See <https://unicode.org/reports/tr44/#Properties>
pub trait UnicodeProperties {
    /// Tests whether this is a Unicode character with the `Pattern_White_space` property.
    ///
    /// See <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3APattern_White_Space%3A%5D&g=&i=>
    fn is_pattern_white_space(&self) -> bool;

    /// Tests whether this is a Unicode character with the `XID_Start` property.
    ///
    /// See <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3AXID_Start%3A%5D%0D%0A%0D%0A%0D%0A&abb=on&g=&i=>
    fn is_xid_start(&self) -> bool;

    /// Tests whether this is a Unicode character with the `XID_Continue` property.
    ///
    /// See <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3AXID_Continue%3A%5D%0D%0A%0D%0A%0D%0A&abb=on&g=&i=>
    fn is_xid_continue(&self) -> bool;
}

const PATTERN_WHITE_SPACE: [char; 11] = [
    '\u{0009}', '\u{000A}', '\u{000B}', '\u{000C}', '\u{000D}', '\u{0020}', '\u{0085}', '\u{200E}',
    '\u{200F}', '\u{2028}', '\u{2029}',
];

impl UnicodeProperties for char {
    fn is_pattern_white_space(&self) -> bool {
        PATTERN_WHITE_SPACE.binary_search(self).is_ok()
    }

    fn is_xid_start(&self) -> bool {
        UnicodeXID::is_xid_start(*self)
    }

    fn is_xid_continue(&self) -> bool {
        UnicodeXID::is_xid_continue(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_white_space() {
        assert!('\u{000B}'.is_pattern_white_space());
        assert!('\n'.is_pattern_white_space());
        assert!(!'a'.is_pattern_white_space());
    }

    #[test]
    fn xid_start() {
        assert!('\u{00F6}'.is_xid_start());
        assert!(!'1'.is_xid_start());
    }

    #[test]
    fn xid_continue() {
        assert!('_'.is_xid_continue());
        assert!(!'\n'.is_xid_continue());
    }
}
