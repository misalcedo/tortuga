
#[cfg(test)]
mod tests {
    use super::*;
    
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
