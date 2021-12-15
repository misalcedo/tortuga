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
    use std::io::sink;

    #[test]
    fn parse_valid_radix_numbers() {
        assert!(pretty_print("36#Z.Z", sink()).is_ok());
        assert!(pretty_print("16#FFFFFF", sink()).is_ok());
        assert!(pretty_print("2#011001", sink()).is_ok());
        assert!(pretty_print("2#+011.101", sink()).is_ok());
        assert!(pretty_print("8#-777", sink()).is_ok());
        assert!(pretty_print("5#4.2", sink()).is_ok());
        assert!(pretty_print("3#.2", sink()).is_ok());
        assert!(pretty_print("1#0.", sink()).is_ok());
    }

    #[test]
    fn parse_invalid_radix_numbers() {
        assert!(pretty_print("7#0.2.5", sink()).is_err());
        assert!(pretty_print("256#Hello", sink()).is_err());
        assert!(pretty_print("0#Hello", sink()).is_err());
        assert!(pretty_print("002#11", sink()).is_err());
        assert!(pretty_print("FF#1", sink()).is_err());
        assert!(pretty_print("+FF#2", sink()).is_err());
        assert!(pretty_print("-FF#4", sink()).is_err());
    }

    #[test]
    fn parse_valid_numbers() {
        assert!(pretty_print("42", sink()).is_ok());
        assert!(pretty_print("0", sink()).is_ok());
        assert!(pretty_print("-5", sink()).is_ok());
        assert!(pretty_print(".5", sink()).is_ok());
        assert!(pretty_print("1.5", sink()).is_ok());
        assert!(pretty_print("+1.2", sink()).is_ok());
        assert!(pretty_print("-1.", sink()).is_ok());
        assert!(pretty_print("+0.2", sink()).is_ok());
        assert!(pretty_print("+1.0", sink()).is_ok());
        assert!(pretty_print("-0.1", sink()).is_ok());
        assert!(pretty_print("-2.0", sink()).is_ok());
        assert!(pretty_print("0.0", sink()).is_ok());
        assert!(pretty_print("0.", sink()).is_ok());
        assert!(pretty_print(".0", sink()).is_ok());
    }

    #[test]
    fn parse_invalid_numbers() {
        assert!(pretty_print("0.2.5", sink()).is_err());
        assert!(pretty_print(".2.5", sink()).is_err());
        assert!(pretty_print("2.5.", sink()).is_err());
        assert!(pretty_print("1 . 2", sink()).is_err());
        assert!(pretty_print("+0", sink()).is_err());
        assert!(pretty_print("-0", sink()).is_err());
        assert!(pretty_print("+0.0", sink()).is_err());
        assert!(pretty_print("-0.0", sink()).is_err());
    }

    #[test]
    fn parse_valid_identifiers() {
        assert!(pretty_print("x2", sink()).is_ok());
        assert!(pretty_print("x_2", sink()).is_ok());
        assert!(pretty_print("x___2", sink()).is_ok());
        assert!(pretty_print("xx", sink()).is_ok());
    }

    #[test]
    fn parse_invalid_identifiers() {
        assert!(pretty_print("2x", sink()).is_err());
        assert!(pretty_print("_x", sink()).is_err());
        assert!(pretty_print("x_", sink()).is_err());
        assert!(pretty_print("x__", sink()).is_err());
        assert!(pretty_print("2_xx", sink()).is_err());
    }
}