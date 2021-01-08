
use crate::graphviz::html::{parse_html, HtmlElement};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{multispace0, space0, space1},
    combinator::{map, opt},
    multi::many1,
    regexp::str::{re_capture, re_find},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub enum Identifier {
    Unquoted(String),
    Quoted(String),
    Numeral(String),
    Html(HtmlElement),
}

/// An ID is one of the following:
/// 1. Any string of alphabetic ([a-zA-Zf \200-\377]) characters, underscores ('_') or digits ([0-9]), not beginning with a digit;
/// 2. a numeral [-]?(.[0-9]+ | [0-9]+(.[0-9]*)? );
/// 3. any double-quoted string ("...") possibly containing escaped quotes (\")1;
/// 4. an HTML string (<...>).
pub(crate) fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let html_parser = map(parse_html, Identifier::Html);
    alt((
        parse_string,
        parse_numeral,
        parse_quoted_string,
        html_parser,
    ))(input)
}

fn parse_string(input: &str) -> IResult<&str, Identifier> {
    let re = Regex::new(r"^[\p{Alphabetic}_]{1}[\p{Alphabetic}_\d]*").unwrap();

    map(re_find(re), |s| Identifier::Unquoted(String::from(s)))(input)
}

/// Parse a quoted string. All characters are valid in quoted strings.
// TODO: allow multi-line strings.
// TODO: allow string concatenation.
fn parse_quoted_string(input: &str) -> IResult<&str, Identifier> {
    let re = Regex::new(r#"^((?:[^\\"]|\\.|\\)*)""#).unwrap();

    map(delimited(tag("\""), re_capture(re), tag("\"")), |s| {
        // The 0 index is the entire match, the 1 index is the first and only capture.
        Identifier::Quoted(String::from(s[1]))
    })(input)
}

/// Parse a numeral identifier token; uses the decimal number system.
/// Allows for positive and negative numerals.
fn parse_numeral(input: &str) -> IResult<&str, Identifier> {
    let re = Regex::new(r"^-?(?:\.\d+|\d+(?:\.\d*)?)").unwrap();

    map(re_find(re), |s| Identifier::Numeral(String::from(s)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphviz::html::TagPosition;

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
