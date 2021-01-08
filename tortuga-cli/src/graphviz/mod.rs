mod attributes;
mod nodes;
mod edges;
mod identifiers;
mod html;
mod graph;

use attributes::*;
use identifiers::*;

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
enum Token {
    // [ Statement [ ';' ] Statements ]
    Statements,
    // node_stmt | edge_stmt | attr_stmt | ID '=' ID | subgraph
    Statement,
    // (node_id | subgraph) edgeRHS [ attr_list ]
    Edge,
    // edgeop (node_id | subgraph) [ edgeRHS ]
    EdgeRHS,
    // node_stmt 	: 	node_id [ attr_list ]
    // node_id 	: 	ID [ port ]
    Node(Identifier, Option<Port>, Option<Attributes>),
    // [ subgraph [ ID ] ] '{' stmt_list '}'
    Subgraph(Option<Identifier>, Vec<Token>),
}

#[derive(Debug, Eq, PartialEq)]
enum Port {
    Identified(Identifier, Option<CompassDirection>),
    Anonymous(CompassDirection),
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

// port	:	':' ID [ ':' compass_pt ] | ':' compass_pt
fn parse_port(input: &str) -> IResult<&str, Port> {
    alt((
        map(
            preceded(terminated(tag(":"), space0), parse_compass_pointer),
            Port::Anonymous,
        ),
        map(
            pair(
                delimited(terminated(tag(":"), space0), parse_identifier, space0),
                opt(preceded(
                    terminated(tag(":"), space0),
                    parse_compass_pointer,
                )),
            ),
            |(identifier, direction)| Port::Identified(identifier, direction),
        ),
    ))(input)
}

// node_stmt 	: 	node_id [ attr_list ]
// node_id 	: 	ID [ port ]
fn parse_node(input: &str) -> IResult<&str, Token> {
    map(
        tuple((parse_identifier, opt(parse_port), opt(parse_attributes))),
        |(identifier, port, attributes)| Token::Node(identifier, port, attributes),
    )(input)
}

// TODO
fn parse_comment(input: &str) -> IResult<&str, ()> { Ok((input, ()))}

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

    #[test]
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

    #[test]
    fn subgraph_invalid() {
        assert!(parse_subgraph("subgraphFoo {}").is_err());
    }

    #[test]
    fn node() {
        assert_eq!(
            parse_node("Pedro:::"),
            Ok((
                ":::",
                Token::Node(Identifier::Unquoted("Pedro".to_string()), None, None)
            ))
        );
        assert_eq!(
            parse_node("Pedro:Foo:"),
            Ok((
                ":",
                Token::Node(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(Port::Identified(
                        Identifier::Unquoted("Foo".to_string()),
                        None
                    )),
                    None
                )
            ))
        );
    }

    #[test]
    fn node_invalid() {
        assert!(parse_node("*").is_err());
    }

    #[test]
    fn port() {
        assert_eq!(
            parse_port(": Pedro:se"),
            Ok((
                "",
                Port::Identified(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(CompassDirection::SouthEast)
                )
            ))
        );
        assert_eq!(
            parse_port(":Pedro:se"),
            Ok((
                "",
                Port::Identified(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(CompassDirection::SouthEast)
                )
            ))
        );
        assert_eq!(
            parse_port(":Pedro: se"),
            Ok((
                "",
                Port::Identified(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(CompassDirection::SouthEast)
                )
            ))
        );
        assert_eq!(
            parse_port(":Pedro : se"),
            Ok((
                "",
                Port::Identified(
                    Identifier::Unquoted("Pedro".to_string()),
                    Some(CompassDirection::SouthEast)
                )
            ))
        );
        assert_eq!(
            parse_port(":Pedro"),
            Ok((
                "",
                Port::Identified(Identifier::Unquoted("Pedro".to_string()), None)
            ))
        );
        assert_eq!(
            parse_port(":Pedro:"),
            Ok((
                ":",
                Port::Identified(Identifier::Unquoted("Pedro".to_string()), None)
            ))
        );
        assert_eq!(
            parse_port(":ne"),
            Ok(("", Port::Anonymous(CompassDirection::NorthEast)))
        );
        assert_eq!(
            parse_port(": ne"),
            Ok(("", Port::Anonymous(CompassDirection::NorthEast)))
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
}
