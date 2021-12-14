//! Uses a PEG grammar to validate a source file.

use crate::CommandLineError;
use pest::Parser;
use pest::iterators::Pair;
use std::io::{stdout, Write};

#[derive(pest_derive::Parser)]
#[grammar = "../docs/grammar.pest"]
pub struct TortugaParser;

///
pub fn validate_file(source: &str) -> Result<(), CommandLineError> {
    let pairs = TortugaParser::parse(Rule::Program, source)?;
    let roots = pairs.into_iter().rev().collect::<Vec<Pair<Rule>>>();
    let root_peers = roots.len();

    let mut stack = Vec::new();

    for pair in roots {
        stack.push((1, root_peers, pair));
    }

    while let Some((depth, peers, pair)) = stack.pop() {
        let rule = pair.as_rule();
        let text = pair.as_str().trim();
        let children = pair.into_inner().into_iter().rev().collect::<Vec<Pair<Rule>>>();
        let children_peers = children.len();

        let mut children_depth = depth;

        if depth == 0 || peers > 1 {
            write!(stdout(), "{0:>1$} ", "-", depth)?;
        }

        write!(stdout(), " {:?}", rule)?;

        match children.len() {
            0 => writeln!(stdout(), ": \"{}\"", text)?,
            1 => write!(stdout(), " →")?,
            _ => {
                children_depth += 2;
                writeln!(stdout(), "")?
            }
        }

        for inner_pair in children {
            stack.push((children_depth, children_peers, inner_pair));
        }
    }

    Ok(())
}