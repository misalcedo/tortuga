
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
