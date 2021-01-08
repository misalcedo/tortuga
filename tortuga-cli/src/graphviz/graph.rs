
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
}
