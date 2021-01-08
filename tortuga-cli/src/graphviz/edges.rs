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
pub enum EdgeOperator {
    Directed,
    Undirected,
}

// An edgeop is -> in directed graphs and -- in undirected graphs.
pub(crate) fn parse_edge_operator(input: &str) -> IResult<&str, EdgeOperator> {
    delimited(
        space0,
    alt((
            map(tag("->"), |_| EdgeOperator::Directed),
            map(tag("--"), |_| EdgeOperator::Undirected),
        )),
        space0
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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
