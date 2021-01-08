
#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::TagPosition;

    #[test]
    fn identifier() {
        assert_eq!(
            parse_identifier("Pedro"),
            Ok(("", Identifier::Unquoted("Pedro".to_string())))
        );
        assert_eq!(
            parse_identifier("Pedro1!"),
            Ok(("!", Identifier::Unquoted("Pedro1".to_string())))
        );
        assert_eq!(
            parse_identifier("\"Pedro\""),
            Ok(("", Identifier::Quoted("Pedro".to_string())))
        );
        assert_eq!(
            parse_identifier("123"),
            Ok(("", Identifier::Numeral("123".to_string())))
        );
        assert_eq!(
            parse_identifier("<p>"),
            Ok((
                "",
                Identifier::Html(HtmlElement::new("p", TagPosition::Open))
            ))
        );
    }

    #[test]
    fn identifier_invalid() {
        assert!(parse_identifier("").is_err());
        assert!(parse_identifier("*").is_err());
    }

    #[test]
    fn unquoted_string() {
        assert_eq!(
            parse_string("Pedro For President!"),
            Ok((" For President!", Identifier::Unquoted("Pedro".to_string())))
        );
        assert_eq!(
            parse_string("Pedro!"),
            Ok(("!", Identifier::Unquoted("Pedro".to_string())))
        );
    }

    #[test]
    fn unquoted_string_underscore() {
        assert_eq!(
            parse_string("_Pedro_ For President!"),
            Ok((
                " For President!",
                Identifier::Unquoted("_Pedro_".to_string())
            ))
        );
    }

    #[test]
    fn unquoted_string_numeric() {
        assert_eq!(
            parse_string("Pedro_123_For_President!"),
            Ok((
                "!",
                Identifier::Unquoted("Pedro_123_For_President".to_string())
            ))
        );
    }

    #[test]
    fn unquoted_string_invalid() {
        assert!(parse_string("123Pedro").is_err());
    }

    #[test]
    fn quoted_string() {
        assert_eq!(
            parse_quoted_string(r#""He\"llo", World!"#),
            Ok((", World!", Identifier::Quoted(r#"He\"llo"#.to_string())))
        );
    }

    #[test]
    fn quoted_string_backslash() {
        assert_eq!(
            parse_quoted_string("\"He\\llo\n\\r\", World!"),
            Ok((", World!", Identifier::Quoted("He\\llo\n\\r".to_string())))
        );
    }

    #[test]
    fn quoted_string_backslash_only() {
        assert_eq!(
            parse_quoted_string("\"Hi\\\""),
            Ok(("", Identifier::Quoted("Hi\\".to_string())))
        );
    }

    #[test]
    fn quoted_string_empty() {
        assert_eq!(
            parse_quoted_string("\"\"Hello, World!"),
            Ok(("Hello, World!", Identifier::Quoted("".to_string())))
        );
    }

    #[test]
    fn quoted_string_invalid() {
        assert!(parse_quoted_string("Hello, World!").is_err());
        assert!(parse_quoted_string("\"Hello, World!").is_err());
        assert!(parse_quoted_string("Hello, World!\"").is_err());
        assert!(parse_quoted_string("Hello\", World!\"").is_err());
        assert!(parse_quoted_string("Hello\"foo\", World!\"").is_err());
    }

    #[test]
    fn numeral() {
        assert_eq!(
            parse_numeral("123"),
            Ok(("", Identifier::Numeral("123".to_string())))
        );
        assert_eq!(
            parse_numeral("123,"),
            Ok((",", Identifier::Numeral("123".to_string())))
        );
    }

    #[test]
    fn numeral_invalid() {
        assert!(parse_numeral(".").is_err());
        assert!(parse_numeral("-.").is_err());
        assert!(parse_numeral("-sdf.").is_err());
        assert!(parse_numeral("-sdf.123").is_err());
    }

    #[test]
    fn numeral_decimal() {
        assert_eq!(
            parse_numeral("123.345"),
            Ok(("", Identifier::Numeral("123.345".to_string())))
        );
        assert_eq!(
            parse_numeral("-123.345"),
            Ok(("", Identifier::Numeral("-123.345".to_string())))
        );
    }

    #[test]
    fn numeral_decimal_no_tail() {
        assert_eq!(
            parse_numeral("123."),
            Ok(("", Identifier::Numeral("123.".to_string())))
        );
        assert_eq!(
            parse_numeral("-123."),
            Ok(("", Identifier::Numeral("-123.".to_string())))
        );
    }

    #[test]
    fn numeral_decimal_no_head() {
        assert_eq!(
            parse_numeral(".123"),
            Ok(("", Identifier::Numeral(".123".to_string())))
        );
        assert_eq!(
            parse_numeral("-.123"),
            Ok(("", Identifier::Numeral("-.123".to_string())))
        );
    }
}
