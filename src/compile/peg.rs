//! Uses a PEG grammar to validate a source file.

use std::io::Write;
use pest::iterators::Pair;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "../docs/grammar.pest"]
struct PegParser;

/// Pretty-print the pest grammar rules.
pub fn pretty_print<'i, O: Write>(source: &str, mut output: O) -> Result<(), ParseError> {
    let pairs = PegParser::parse(Rule::Program, source)?;
    let roots = pairs.into_iter().rev().collect::<Vec<Pair<Rule>>>();
    let root_peers = roots.len();

    let mut stack = Vec::new();

    for pair in roots {
        stack.push((1, root_peers, pair));
    }

    while let Some((depth, peers, pair)) = stack.pop() {
        let rule = pair.as_rule();
        let text = pair.as_str().trim();
        let children = pair
            .into_inner()
            .into_iter()
            .rev()
            .collect::<Vec<Pair<Rule>>>();
        let children_peers = children.len();

        let mut children_depth = depth;

        if depth == 0 || peers > 1 {
            write!(output, "{0:>1$} ", "-", depth)?;
        }

        write!(output, " {:?}", rule)?;

        match children.len() {
            0 => writeln!(output, ": \"{}\"", text)?,
            1 => write!(output, " →")?,
            _ => {
                children_depth += 2;
                writeln!(output, "")?
            }
        }

        for inner_pair in children {
            stack.push((children_depth, children_peers, inner_pair));
        }
    }

    Ok(())
}

/// An error that occurred while parsing using a PEG-generated parser.
#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Validation(#[from] pest::error::Error<Rule>)
}

#[cfg(test)]
mod tests {
    use super::pretty_print;
    use crate::Parser;
    use std::io::sink;

    fn validate(source: &str, predicate: fn(&Result<(), ()>) -> bool) {
        let peg_result = predicate(&pretty_print(source, sink()).map_err(|_| ()));
        let parser_result = predicate(&Parser::default().parse(source).map(|_| ()).map_err(|_| ()));

        assert_eq!(peg_result, parser_result,"PEG-Generated and hand-written parser (respectively) do not agree on the validity of the given source: '{}'.", source);
    }

    #[test]
    fn parse_valid_radix_numbers() {
        validate("36#Z.Z", Result::is_ok);

        validate("36#Z.Z", Result::is_ok);
        validate("16#FFFFFF", Result::is_ok);
        validate("2#011001", Result::is_ok);
        validate("2#+011.101", Result::is_ok);
        validate("8#-777", Result::is_ok);
        validate("5#4.2", Result::is_ok);
        validate("3#.2", Result::is_ok);
        validate("1#0.", Result::is_ok);
    }

    #[test]
    fn parse_invalid_radix_numbers() {
        validate("7#0.2.5", Result::is_err);
        validate("256#Hello", Result::is_err);
        validate("0#Hello", Result::is_err);
        validate("002#11", Result::is_err);
        validate("FF#1", Result::is_err);
        validate("+FF#2", Result::is_err);
        validate("-FF#4", Result::is_err);
    }

    #[test]
    fn parse_valid_numbers() {
        validate("42", Result::is_ok);
        validate("0", Result::is_ok);
        validate("-5", Result::is_ok);
        validate(".5", Result::is_ok);
        validate("1.5", Result::is_ok);
        validate("+1.2", Result::is_ok);
        validate("-1.", Result::is_ok);
        validate("+0.2", Result::is_ok);
        validate("+1.0", Result::is_ok);
        validate("-0.1", Result::is_ok);
        validate("-2.0", Result::is_ok);
        validate("0.0", Result::is_ok);
        validate("0.", Result::is_ok);
        validate(".0", Result::is_ok);
    }

    #[test]
    fn parse_invalid_numbers() {
        validate("0.2.5", Result::is_err);
        validate(".2.5", Result::is_err);
        validate("2.5.", Result::is_err);
        validate("1 . 2", Result::is_err);
        validate("+0", Result::is_err);
        validate("-0", Result::is_err);
        validate("+0.0", Result::is_err);
        validate("-0.0", Result::is_err);
    }

    #[test]
    fn parse_valid_identifiers() {
        validate("x2", Result::is_ok);
        validate("x_2", Result::is_ok);
        validate("x___2", Result::is_ok);
        validate("xx", Result::is_ok);
    }

    #[test]
    fn parse_invalid_identifiers() {
        validate("2x", Result::is_err);
        validate("_x", Result::is_err);
        validate("x_", Result::is_err);
        validate("x__", Result::is_err);
        validate("2_xx", Result::is_err);
    }
}