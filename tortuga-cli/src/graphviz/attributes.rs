use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{multispace0, space0, space1},
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use crate::graphviz::identifiers::{Identifier, parse_identifier};

#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    Graph,
    Node,
    Edge,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Attributes(Kind, Vec<Vec<Attribute>>);

#[derive(Debug, Eq, PartialEq)]
pub struct Attribute(Identifier, Identifier);


/// attr_stmt	:	(graph | node | edge) attr_list
/// attr_list	:	'[' [ a_list ] ']' [ attr_list ]
pub(crate) fn parse_attributes(input: &str) -> IResult<&str, Attributes> {
    map(
        pair(parse_kind, parse_attribute_list),
        |(kind, attributes)| Attributes(kind, attributes),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes() {
        assert_eq!(
            parse_attributes("edge [] [] [] []"),
            Ok(("", Attributes(Kind::Edge, vec![])))
        );
        assert_eq!(
            parse_attributes("graph [Pedro1=Pedro2]"),
            Ok((
                "",
                Attributes(
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
                Attributes(
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
}
