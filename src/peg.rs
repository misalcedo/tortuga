//! Uses a PEG grammar to validate a source file.

use pest::iterators::Pair;
use pest::Parser;
use std::io::Write;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
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
    Validation(#[from] pest::error::Error<Rule>),
}

#[cfg(test)]
mod tests {
    use super::pretty_print;
    use std::io::sink;

    #[test]
    fn parse() {
        assert!(pretty_print("1#0.", sink()).is_ok());
    }
}
