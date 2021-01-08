mod attributes;
mod nodes;
mod edges;
mod identifiers;
mod html;

use attributes::*;
use identifiers::*;
use nodes::*;
use edges::*;

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
    NodeStatement(Node),
    // [ subgraph [ ID ] ] '{' stmt_list '}'
    Subgraph(Option<Identifier>, Vec<Token>),
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

// TODO
fn parse_comment(input: &str) -> IResult<&str, ()> { Ok((input, ()))}

#[cfg(test)]
mod tests {
    use super::*;

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
}
