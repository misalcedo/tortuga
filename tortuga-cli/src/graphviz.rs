use crate::html::{parse_html, HtmlElement};
use regex::Regex;
use nom::{
    IResult,
    bytes::complete::{tag, tag_no_case, take_while},
    branch::alt,
    combinator::map,
    regexp::str::re_find,
    sequence::delimited};
    
#[derive(Debug, Eq, PartialEq)]
struct Graph {

}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    // [ strict ] (graph | digraph) [ ID ] '{' Statements '}'
    Graph,
    // [ Statement [ ';' ] Statements ]
    Statements,	
    // node_stmt | edge_stmt | attr_stmt | ID '=' ID | subgraph
    Statement,
    // (graph | node | edge) Attributes	
    Attribute,
    // '[' [ a_list ] ']' [ Attributes ]
    Attributes,
    // ID '=' ID [ (';' | ',') ] [ a_list ]
    AList(Identifier),
    // (node_id | subgraph) edgeRHS [ attr_list ]
    Edge,
    // edgeop (node_id | subgraph) [ edgeRHS ]
    EdgeRHS,
    // node_id [ attr_list ]
    Node,
    // ID [ port ]
    NodeIdentifier(Identifier),
    // ':' ID [ ':' compass_pt ] | ':' compass_pt
    Port,
    // [ subgraph [ ID ] ] '{' stmt_list '}'
    Subgraph,
    // (n | ne | e | se | s | sw | w | nw | c | _)
    CompassPointer(CompassDirection)
}

#[derive(Debug, Eq, PartialEq)]
enum Identifier {
    Unquoted(String),
    Quoted(String),
    Numeral(String),
    Html(HtmlElement)
}

#[derive(Debug, Eq, PartialEq)]
enum CompassDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Center,
    Underscore,
    Any(Identifier)
}

impl Graph {
  fn new() -> Graph {
      Graph {
      }
  }
}

/// Parse a DOT language file into the corresponding graph.
/// See https://graphviz.org/doc/info/lang.html
fn parse(input: &str) -> IResult<&str, Graph> {
    let system = Graph::new();
    Ok((input, system))
}

fn parse_compass_pointer(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag_no_case("ne"), |_| Token::CompassPointer(CompassDirection::NorthEast)),
        map(tag_no_case("se"), |_| Token::CompassPointer(CompassDirection::SouthEast)),
        map(tag_no_case("sw"), |_| Token::CompassPointer(CompassDirection::SouthWest)),
        map(tag_no_case("nw"), |_| Token::CompassPointer(CompassDirection::NorthWest)),
        map(tag_no_case("n"), |_| Token::CompassPointer(CompassDirection::North)),
        map(tag_no_case("e"), |_| Token::CompassPointer(CompassDirection::East)),
        map(tag_no_case("s"), |_| Token::CompassPointer(CompassDirection::South)),
        map(tag_no_case("w"), |_| Token::CompassPointer(CompassDirection::West)),
        map(tag_no_case("c"), |_| Token::CompassPointer(CompassDirection::Center)),
        map(tag("_"), |_| Token::CompassPointer(CompassDirection::Underscore)),
        map(parse_identifier, |i| Token::CompassPointer(CompassDirection::Any(i)))
    ))(input)
}

/// An ID is one of the following:
/// 1. Any string of alphabetic ([a-zA-Zf \200-\377]) characters, underscores ('_') or digits ([0-9]), not beginning with a digit;
/// 2. a numeral [-]?(.[0-9]+ | [0-9]+(.[0-9]*)? );
/// 3. any double-quoted string ("...") possibly containing escaped quotes (\")1;
/// 4. an HTML string (<...>).
fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let html_parser = map(parse_html, |element| Identifier::Html(element));
    alt((parse_string, parse_numeral, parse_quoted_string, html_parser))(input)
}

fn parse_string(input: &str) -> IResult<&str, Identifier> {
    let re = Regex::new(r"^[\p{Alphabetic}_]{1}[\p{Alphabetic}_\d]*").unwrap();

    map(
        re_find(re),
        |s| Identifier::Unquoted(String::from(s))
    )(input)
}

/// Parse a quoted string. Any characters are alllowed in the quoted string.
fn parse_quoted_string(input: &str) -> IResult<&str, Identifier> {
    let re = Regex::new(r#"(?:[^\\"]|\\.|\\)*"#).unwrap();

    map(
        delimited(tag("\""), re_find(re), tag("\"")),
        |s| Identifier::Quoted(String::from(s))
    )(input)
}

/// Parse a numeral identifier token; uses the decimal number system.
/// Allows for positive and negative numerals.
fn parse_numeral(input: &str) -> IResult<&str, Identifier> {
    let re = Regex::new(r"^-?(?:\.\d+|\d+(?:\.\d*)?)").unwrap();

    map(
        re_find(re),
        |s| Identifier::Numeral(String::from(s))
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::TagPosition;

    #[test]
    fn empty_graph() {
    }

    #[test]
    fn compass_direction() {
        assert_eq!(parse_compass_pointer("n"), Ok(("", Token::CompassPointer(CompassDirection::North))));
        assert_eq!(parse_compass_pointer("nE"), Ok(("", Token::CompassPointer(CompassDirection::NorthEast))));
        assert_eq!(parse_compass_pointer("e"), Ok(("", Token::CompassPointer(CompassDirection::East))));
        assert_eq!(parse_compass_pointer("Se"), Ok(("", Token::CompassPointer(CompassDirection::SouthEast))));
        assert_eq!(parse_compass_pointer("s"), Ok(("", Token::CompassPointer(CompassDirection::South))));
        assert_eq!(parse_compass_pointer("SW"), Ok(("", Token::CompassPointer(CompassDirection::SouthWest))));
        assert_eq!(parse_compass_pointer("w"), Ok(("", Token::CompassPointer(CompassDirection::West))));
        assert_eq!(parse_compass_pointer("nw"), Ok(("", Token::CompassPointer(CompassDirection::NorthWest))));
        assert_eq!(parse_compass_pointer("c"), Ok(("", Token::CompassPointer(CompassDirection::Center))));
        assert_eq!(parse_compass_pointer("_"), Ok(("", Token::CompassPointer(CompassDirection::Underscore))));
        assert_eq!(parse_compass_pointer("Pedro"), Ok(("", Token::CompassPointer(CompassDirection::Any(Identifier::Unquoted("Pedro".to_string()))))));
    }

    #[test]
    fn compass_direction_invalid() {
        assert!(parse_compass_pointer("*").is_err());
    }

    #[test]
    fn identifier() {
        assert_eq!(parse_identifier("Pedro"), Ok(("", Identifier::Unquoted("Pedro".to_string()))));
        assert_eq!(parse_identifier("\"Pedro\""), Ok(("", Identifier::Quoted("Pedro".to_string()))));
        assert_eq!(parse_identifier("123"), Ok(("", Identifier::Numeral("123".to_string()))));
        assert_eq!(parse_identifier("<p>"), Ok(("", Identifier::Html(HtmlElement::new("p", TagPosition::Open)))));
    }

    #[test]
    fn unquoted_string() {
        assert_eq!(parse_string("Pedro For President!"), Ok((" For President!", Identifier::Unquoted("Pedro".to_string()))));
    }

    #[test]
    fn unquoted_string_underscore() {
        assert_eq!(parse_string("_Pedro_ For President!"), Ok((" For President!", Identifier::Unquoted("_Pedro_".to_string()))));
    }

    #[test]
    fn unquoted_string_numeric() {
        assert_eq!(parse_string("Pedro_123_For_President!"), Ok(("!", Identifier::Unquoted("Pedro_123_For_President".to_string()))));
    }

    #[test]
    fn unquoted_string_invalid() {
        assert!(parse_string("123Pedro").is_err());
    }

    #[test]
    fn quoted_string() {
        assert_eq!(parse_quoted_string(r#""He\"llo", World!"#), Ok((", World!", Identifier::Quoted(r#"He\"llo"#.to_string()))));
    }

    #[test]
    fn quoted_string_backslash() {
        assert_eq!(parse_quoted_string("\"He\\llo\n\\r\", World!"), Ok((", World!", Identifier::Quoted("He\\llo\n\\r".to_string()))));
    }

    #[test]
    fn quoted_string_empty() {
        assert_eq!(parse_quoted_string("\"\"Hello, World!"), Ok(("Hello, World!", Identifier::Quoted("".to_string()))));
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
        assert_eq!(parse_numeral("123"), Ok(("", Identifier::Numeral("123".to_string()))));
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
        assert_eq!(parse_numeral("123.345"), Ok(("", Identifier::Numeral("123.345".to_string()))));
        assert_eq!(parse_numeral("-123.345"), Ok(("", Identifier::Numeral("-123.345".to_string()))));
    }

    #[test]
    fn numeral_decimal_no_tail() {
        assert_eq!(parse_numeral("123."), Ok(("", Identifier::Numeral("123.".to_string()))));
        assert_eq!(parse_numeral("-123."), Ok(("", Identifier::Numeral("-123.".to_string()))));
    }

    #[test]
    fn numeral_decimal_no_head() {
        assert_eq!(parse_numeral(".123"), Ok(("", Identifier::Numeral(".123".to_string()))));
        assert_eq!(parse_numeral("-.123"), Ok(("", Identifier::Numeral("-.123".to_string()))));
    }
}