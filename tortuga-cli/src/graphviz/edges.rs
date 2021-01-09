use crate::graphviz::attributes::{parse_attribute_list, AttributeList};
use crate::graphviz::identifiers::Identifier;
use crate::graphviz::nodes::{parse_node_id, NodeId};
use crate::graphviz::{parse_subgraph, Token};
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

#[derive(Debug, Eq, PartialEq)]
pub enum EdgeOperator {
    Directed,
    Undirected,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Edge(EdgeId, Vec<EdgeTarget>, Option<AttributeList>);

#[derive(Debug, Eq, PartialEq)]
pub struct EdgeTarget(EdgeOperator, EdgeId);

#[derive(Debug, Eq, PartialEq)]
pub enum EdgeId {
    Node(NodeId),
    Subgraph(Token),
}

// edge_stmt	:	(node_id | subgraph) edgeRHS [ attr_list ]
// edgeRHS	:	edgeop (node_id | subgraph) [ edgeRHS ]
pub fn parse_edges(input: &str) -> IResult<&str, Edge> {
    map(
        tuple((parse_edge_id, parse_edge_targets, opt(preceded(space0, parse_attribute_list)))),
        |(id, targets, attributes)| Edge(id, targets, attributes),
    )(input)
}

fn parse_edge_targets(input: &str) -> IResult<&str, Vec<EdgeTarget>> {
    many1(map(pair(parse_edge_operator, parse_edge_id), |(op, id)| {
        EdgeTarget(op, id)
    }))(input)
}

fn parse_edge_id(input: &str) -> IResult<&str, EdgeId> {
    alt((
        map(parse_subgraph, EdgeId::Subgraph),
        map(parse_node_id, EdgeId::Node),
    ))(input)
}

// An edgeop is -> in directed graphs and -- in undirected graphs.
fn parse_edge_operator(input: &str) -> IResult<&str, EdgeOperator> {
    delimited(
        space0,
        alt((
            map(tag("->"), |_| EdgeOperator::Directed),
            map(tag("--"), |_| EdgeOperator::Undirected),
        )),
        space0,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        assert_eq!(
            parse_edges("A -> B -> C -- D"),
            Ok((
                "",
                Edge(
                    EdgeId::Node(NodeId(Identifier::Unquoted("A".to_string()), None)),
                    vec![
                        EdgeTarget(
                            EdgeOperator::Directed,
                            EdgeId::Node(NodeId(Identifier::Unquoted("B".to_string()), None))
                        ),
                        EdgeTarget(
                            EdgeOperator::Directed,
                            EdgeId::Node(NodeId(Identifier::Unquoted("C".to_string()), None))
                        ),
                        EdgeTarget(
                            EdgeOperator::Undirected,
                            EdgeId::Node(NodeId(Identifier::Unquoted("D".to_string()), None))
                        ),
                    ],
                    None
                )
            ))
        );
        assert_eq!(
            parse_edges("A -- subgraph B {} -> C []"),
            Ok((
                "",
                Edge(
                    EdgeId::Node(NodeId(Identifier::Unquoted("A".to_string()), None)),
                    vec![
                        EdgeTarget(
                            EdgeOperator::Undirected,
                            EdgeId::Subgraph(Token::Subgraph(
                                Some(Identifier::Unquoted("B".to_string())),
                                vec![]
                            ))
                        ),
                        EdgeTarget(
                            EdgeOperator::Directed,
                            EdgeId::Node(NodeId(Identifier::Unquoted("C".to_string()), None))
                        )
                    ],
                    Some(AttributeList(vec![]))
                )
            ))
        );
        assert_eq!(
            parse_edges("A -- {} -> C {} --"),
            Ok((
                " {} --",
                Edge(
                    EdgeId::Node(NodeId(Identifier::Unquoted("A".to_string()), None)),
                    vec![
                        EdgeTarget(
                            EdgeOperator::Undirected,
                            EdgeId::Subgraph(Token::Subgraph(None, vec![]))
                        ),
                        EdgeTarget(
                            EdgeOperator::Directed,
                            EdgeId::Node(NodeId(Identifier::Unquoted("C".to_string()), None))
                        )
                    ],
                    None
                )
            ))
        );
    }

    #[test]
    fn edges_invalid() {
        assert!(parse_edges("A ->").is_err());
    }

    #[test]
    fn edgeop() {
        assert_eq!(
            parse_edge_operator("  -> "),
            Ok(("", EdgeOperator::Directed))
        );
        assert_eq!(
            parse_edge_operator(" --  "),
            Ok(("", EdgeOperator::Undirected))
        );
        assert_eq!(
            parse_edge_operator(" ---"),
            Ok(("-", EdgeOperator::Undirected))
        );
    }

    #[test]
    fn edgeop_invalid() {
        assert!(parse_edge_operator("<->").is_err());
        assert!(parse_edge_operator("<>").is_err());
    }
}
