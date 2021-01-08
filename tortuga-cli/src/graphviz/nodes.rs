
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
use crate::graphviz::attributes::{Attributes, parse_attributes};

#[derive(Debug, Eq, PartialEq)]
pub enum Port {
    Identified(Identifier, Option<CompassDirection>),
    Anonymous(CompassDirection),
}

#[derive(Debug, Eq, PartialEq)]
pub enum CompassDirection {
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
pub struct Node(Identifier, Option<Port>, Option<Attributes>);

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
pub(crate) fn parse_node(input: &str) -> IResult<&str, Node> {
    map(
        tuple((parse_identifier, opt(parse_port), opt(parse_attributes))),
        |(identifier, port, attributes)| Node(identifier, port, attributes),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn node() {
        assert_eq!(
            parse_node("Pedro:::"),
            Ok((
                ":::",
                Node(Identifier::Unquoted("Pedro".to_string()), None, None)
            ))
        );
        assert_eq!(
            parse_node("Pedro:Foo:"),
            Ok((
                ":",
                Node(
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
