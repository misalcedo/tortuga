use crate::html::{parse_html, HtmlElement};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while},
    character::complete::{line_ending, multispace0, multispace1, space0, space1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    regexp::str::re_find,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Graph {
    strict: bool,
    kind: GraphKind,
    identifier: Option<Identifier>,
    statements: Vec<Token>,
}

impl Graph {
    fn new(
        strict: bool,
        kind: GraphKind,
        identifier: Option<Identifier>,
        statements: Vec<Token>,
    ) -> Graph {
        Graph {
            strict,
            kind,
            identifier,
            statements,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Kind {
    Graph,
    Node,
    Edge,
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    // [ Statement [ ';' ] Statements ]
    Statements,
    // node_stmt | edge_stmt | attr_stmt | ID '=' ID | subgraph
    Statement,
    Attributes(Kind, Vec<Vec<Attribute>>),
    // (node_id | subgraph) edgeRHS [ attr_list ]
    Edge,
    // edgeop (node_id | subgraph) [ edgeRHS ]
    EdgeRHS,
    // node_id [ attr_list ]
    Node(Identifier, Option<Identifier>),
    IdentifiedPort(Identifier, Option<CompassDirection>),
    AnonymousPort(CompassDirection),
    // [ subgraph [ ID ] ] '{' stmt_list '}'
    Subgraph(Option<Identifier>, Vec<Token>),
}

#[derive(Debug, Eq, PartialEq)]
enum Identifier {
    Unquoted(String),
    Quoted(String),
    Numeral(String),
    Html(HtmlElement),
}

#[derive(Debug, Eq, PartialEq)]
struct Attribute(Identifier, Identifier);

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
}

#[derive(Debug, Eq, PartialEq)]
enum GraphKind {
    Graph,
    DiGraph,
}

/// Parse a DOT language file into the corresponding graph.
/// See https://graphviz.org/doc/info/lang.html
// [ strict ] (graph | digraph) [ ID ] '{' Statements '}'
fn parse(input: &str) -> IResult<&str, Graph> {
    map(
        preceded(
            multispace0,
            tuple((
                opt(terminated(tag("strict"), space1)),
                alt((
                    map(tag("graph"), |_| GraphKind::Graph),
                    map(tag("digraph"), |_| GraphKind::DiGraph),
                )),
                delimited(space1, opt(parse_identifier), multispace0),
                delimited(
                    preceded(tag("{"), multispace0),
                    parse_statements,
                    terminated(tag("}"), multispace0),
                ),
            )),
        ),
        |(strict, kind, identifier, statements)| Graph {
            strict: strict.is_some(),
            identifier,
            kind,
            statements,
        },
    )(input)
}

// [ subgraph [ ID ] ] '{' stmt_list '}'
fn parse_subgraph(input: &str) -> IResult<&str, Token> {
    map(
        pair(
            delimited(
                multispace0,
                opt(preceded(
                    terminated(tag("subgraph"), space1),
                    opt(parse_identifier),
                )),
                multispace0,
            ),
            delimited(
                preceded(tag("{"), multispace0),
                parse_statements,
                terminated(tag("}"), multispace0),
            ),
        ),
        |(id, statements)| Token::Subgraph(id.flatten(), statements),
    )(input)
}

fn parse_statements(input: &str) -> IResult<&str, Vec<Token>> {
    Ok((input, vec![]))
}

fn parse_compass_pointer(input: &str) -> IResult<&str, CompassDirection> {
    alt((
        map(tag_no_case("ne"), |_| CompassDirection::NorthEast),
        map(tag_no_case("se"), |_| CompassDirection::SouthEast),
        map(tag_no_case("sw"), |_| CompassDirection::SouthWest),
        map(tag_no_case("nw"), |_| CompassDirection::NorthWest),
        map(tag_no_case("n"), |_| CompassDirection::North),
        map(tag_no_case("e"), |_| CompassDirection::East),
        map(tag_no_case("s"), |_| CompassDirection::South),
        map(tag_no_case("w"), |_| CompassDirection::West),
        map(tag_no_case("c"), |_| CompassDirection::Center),
        map(tag("_"), |_| CompassDirection::Underscore),
    ))(input)
}

/// An ID is one of the following:
/// 1. Any string of alphabetic ([a-zA-Zf \200-\377]) characters, underscores ('_') or digits ([0-9]), not beginning with a digit;
/// 2. a numeral [-]?(.[0-9]+ | [0-9]+(.[0-9]*)? );
/// 3. any double-quoted string ("...") possibly containing escaped quotes (\")1;
/// 4. an HTML string (<...>).
fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let html_parser = map(parse_html, |element| Identifier::Html(element));
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

/// Parse a quoted string. Any characters are alllowed in the quoted string.
fn parse_quoted_string(input: &str) -> IResult<&str, Identifier> {
    let re = Regex::new(r#"(?:[^\\"]|\\.|\\)*"#).unwrap();

    map(delimited(tag("\""), re_find(re), tag("\"")), |s| {
        Identifier::Quoted(String::from(s))
    })(input)
}

/// Parse a numeral identifier token; uses the decimal number system.
/// Allows for positive and negative numerals.
fn parse_numeral(input: &str) -> IResult<&str, Identifier> {
    let re = Regex::new(r"^-?(?:\.\d+|\d+(?:\.\d*)?)").unwrap();

    map(re_find(re), |s| Identifier::Numeral(String::from(s)))(input)
}

/// attr_stmt	:	(graph | node | edge) attr_list
/// attr_list	:	'[' [ a_list ] ']' [ attr_list ]
fn parse_attributes(input: &str) -> IResult<&str, Token> {
    map(
        pair(parse_kind, parse_attribute_list),
        |(kind, attributes)| Token::Attributes(kind, attributes),
    )(input)
}

fn parse_attribute_list(input: &str) -> IResult<&str, Vec<Vec<Attribute>>> {
    map(
        many1(delimited(
            terminated(tag("["), multispace0),
            opt(parse_attribute),
            terminated(tag("]"), multispace0),
        )),
        |attributes| attributes.into_iter().flatten().collect(),
    )(input)
}

fn parse_kind(input: &str) -> IResult<&str, Kind> {
    terminated(
        alt((
            map(tag("graph"), |_| Kind::Graph),
            map(tag("node"), |_| Kind::Node),
            map(tag("edge"), |_| Kind::Edge),
        )),
        multispace0,
    )(input)
}

/// a_list	:	ID '=' ID [ (';' | ',') ] [ a_list ]
fn parse_attribute(input: &str) -> IResult<&str, Vec<Attribute>> {
    many1(map(
        separated_pair(
            parse_identifier,
            delimited(space0, tag("="), space0),
            terminated(
                parse_identifier,
                terminated(
                    opt(preceded(space0, alt((tag(";"), tag(","))))),
                    multispace0,
                ),
            ),
        ),
        |(a, b)| Attribute(a, b),
    ))(input)
}

// port	:	':' ID [ ':' compass_pt ] | ':' compass_pt
fn parse_port(input: &str) -> IResult<&str, Token> {
    alt((
        map(
            preceded(terminated(tag(":"), space0), parse_compass_pointer),
            |direction| Token::AnonymousPort(direction),
        ),
        map(
            pair(
                delimited(terminated(tag(":"), space0), parse_identifier, space0),
                opt(preceded(
                    terminated(tag(":"), space0),
                    parse_compass_pointer,
                )),
            ),
            |(identifier, direction)| Token::IdentifiedPort(identifier, direction),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::TagPosition;

    #[test]
    fn graph() {
        assert_eq!(
            parse("strict digraph {}"),
            Ok(("", Graph::new(true, GraphKind::DiGraph, None, vec![])))
        );
        assert_eq!(
            parse("graph 123 {}"),
            Ok((
                "",
                Graph::new(
                    false,
                    GraphKind::Graph,
                    Some(Identifier::Numeral("123".to_string())),
                    vec![]
                )
            ))
        );
        assert_eq!(
            parse("graph {}"),
            Ok(("", Graph::new(false, GraphKind::Graph, None, vec![])))
        );
    }

    fn graph_invalid() {
        assert!(parse("strict graphFoo {}").is_err());
    }
    #[test]
    fn subgraph() {
        assert_eq!(
            parse_subgraph("subgraph {}"),
            Ok(("", Token::Subgraph(None, vec![])))
        );

        assert_eq!(
            parse_subgraph("subgraph Pedro {}"),
            Ok((
                "",
                Token::Subgraph(Some(Identifier::Unquoted("Pedro".to_string())), vec![])
            ))
        );
    }

    fn subgraph_invalid() {
        assert!(parse_subgraph("subgraphFoo {}").is_err());
    }

    #[test]
    fn attributes() {
        assert_eq!(
            parse_attributes("edge [] [] [] []"),
            Ok(("", Token::Attributes(Kind::Edge, vec![])))
        );
        assert_eq!(
            parse_attributes("graph [Pedro1=Pedro2]"),
            Ok((
                "",
                Token::Attributes(
                    Kind::Graph,
                    vec![vec![Attribute(
                        Identifier::Unquoted("Pedro1".to_string()),
                        Identifier::Unquoted("Pedro2".to_string())
                    )]]
                )
            ))
        );
        assert_eq!(
            parse_attributes("node [Pedro1 = Pedro2 A=B;]"),
            Ok((
                "",
                Token::Attributes(
                    Kind::Node,
                    vec![vec![
                        Attribute(
                            Identifier::Unquoted("Pedro1".to_string()),
                            Identifier::Unquoted("Pedro2".to_string())
                        ),
                        Attribute(
                            Identifier::Unquoted("A".to_string()),
                            Identifier::Unquoted("B".to_string())
                        )
                    ]]
                )
            ))
        );
    }

    #[test]
    fn attributes_invalid() {
        assert!(parse_attributes("fudge [] [] [] []").is_err());
        assert!(parse_attributes("edge {}").is_err());
        assert!(parse_attributes("graph [Pedro1|Pedro2]").is_err());
    }

    #[test]
    fn attribute() {
        assert_eq!(
            parse_attribute("Pedro1=Pedro2"),
            Ok((
                "",
                vec![Attribute(
                    Identifier::Unquoted("Pedro1".to_string()),
                    Identifier::Unquoted("Pedro2".to_string())
                )]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1= Pedro2"),
            Ok((
                "",
                vec![Attribute(
                    Identifier::Unquoted("Pedro1".to_string()),
                    Identifier::Unquoted("Pedro2".to_string())
                )]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1 = Pedro2"),
            Ok((
                "",
                vec![Attribute(
                    Identifier::Unquoted("Pedro1".to_string()),
                    Identifier::Unquoted("Pedro2".to_string())
                )]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1 = Pedro2"),
            Ok((
                "",
                vec![Attribute(
                    Identifier::Unquoted("Pedro1".to_string()),
                    Identifier::Unquoted("Pedro2".to_string())
                )]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1 = Pedro2;"),
            Ok((
                "",
                vec![Attribute(
                    Identifier::Unquoted("Pedro1".to_string()),
                    Identifier::Unquoted("Pedro2".to_string())
                )]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1 = Pedro2  ,"),
            Ok((
                "",
                vec![Attribute(
                    Identifier::Unquoted("Pedro1".to_string()),
                    Identifier::Unquoted("Pedro2".to_string())
                )]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1 = Pedro2 A=B;"),
            Ok((
                "",
                vec![
                    Attribute(
                        Identifier::Unquoted("Pedro1".to_string()),
                        Identifier::Unquoted("Pedro2".to_string())
                    ),
                    Attribute(
                        Identifier::Unquoted("A".to_string()),
                        Identifier::Unquoted("B".to_string())
                    )
                ]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1 = Pedro2  , \nA=B;  "),
            Ok((
                "",
                vec![
                    Attribute(
                        Identifier::Unquoted("Pedro1".to_string()),
                        Identifier::Unquoted("Pedro2".to_string())
                    ),
                    Attribute(
                        Identifier::Unquoted("A".to_string()),
                        Identifier::Unquoted("B".to_string())
                    )
                ]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1 = Pedro2\nA=B;  "),
            Ok((
                "",
                vec![
                    Attribute(
                        Identifier::Unquoted("Pedro1".to_string()),
                        Identifier::Unquoted("Pedro2".to_string())
                    ),
                    Attribute(
                        Identifier::Unquoted("A".to_string()),
                        Identifier::Unquoted("B".to_string())
                    )
                ]
            ))
        );
        assert_eq!(
            parse_attribute("Pedro1 = Pedro2A=B;  "),
            Ok((
                "=B;  ",
                vec![Attribute(
                    Identifier::Unquoted("Pedro1".to_string()),
                    Identifier::Unquoted("Pedro2A".to_string())
                )]
            ))
        );
    }

    #[test]
    fn attribute_invalid() {
        assert!(parse_attribute("* = foo,").is_err());
    }

    #[test]
    fn port() {
        assert_eq!(
            parse_port(": Pedro:se"),
            Ok((
                "",
                Token::IdentifiedPort(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(CompassDirection::SouthEast)
                )
            ))
        );
        assert_eq!(
            parse_port(":Pedro:se"),
            Ok((
                "",
                Token::IdentifiedPort(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(CompassDirection::SouthEast)
                )
            ))
        );
        assert_eq!(
            parse_port(":Pedro: se"),
            Ok((
                "",
                Token::IdentifiedPort(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(CompassDirection::SouthEast)
                )
            ))
        );
        assert_eq!(
            parse_port(":Pedro : se"),
            Ok((
                "",
                Token::IdentifiedPort(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(CompassDirection::SouthEast)
                )
            ))
        );
        assert_eq!(
            parse_port(":Pedro"),
            Ok((
                "",
                Token::IdentifiedPort(Identifier::Unquoted("Pedro".to_string()), None)
            ))
        );
        assert_eq!(
            parse_port(":Pedro:"),
            Ok((
                ":",
                Token::IdentifiedPort(Identifier::Unquoted("Pedro".to_string()), None)
            ))
        );
        assert_eq!(
            parse_port(":ne"),
            Ok(("", Token::AnonymousPort(CompassDirection::NorthEast)))
        );
        assert_eq!(
            parse_port(": ne"),
            Ok(("", Token::AnonymousPort(CompassDirection::NorthEast)))
        );
    }

    #[test]
    fn port_invalid() {
        assert!(parse_port(":").is_err());
    }

    #[test]
    fn compass_direction() {
        assert_eq!(
            parse_compass_pointer("n"),
            Ok(("", CompassDirection::North))
        );
        assert_eq!(
            parse_compass_pointer("nE"),
            Ok(("", CompassDirection::NorthEast))
        );
        assert_eq!(parse_compass_pointer("e"), Ok(("", CompassDirection::East)));
        assert_eq!(
            parse_compass_pointer("Se"),
            Ok(("", CompassDirection::SouthEast))
        );
        assert_eq!(
            parse_compass_pointer("s"),
            Ok(("", CompassDirection::South))
        );
        assert_eq!(
            parse_compass_pointer("SW"),
            Ok(("", CompassDirection::SouthWest))
        );
        assert_eq!(parse_compass_pointer("w"), Ok(("", CompassDirection::West)));
        assert_eq!(
            parse_compass_pointer("nw"),
            Ok(("", CompassDirection::NorthWest))
        );
        assert_eq!(
            parse_compass_pointer("c"),
            Ok(("", CompassDirection::Center))
        );
        assert_eq!(
            parse_compass_pointer("_"),
            Ok(("", CompassDirection::Underscore))
        );
    }

    #[test]
    fn compass_direction_invalid() {
        assert!(parse_compass_pointer("*").is_err());
    }

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
