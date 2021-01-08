
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
