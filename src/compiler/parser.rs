//! Parse a sequence of tokens into a syntax tree.
//! 
//! Relies on a Parser Expression Grammar to generate a parser for the language.
//! 
//! See <https://en.wikipedia.org/wiki/Parsing_expression_grammar>

use crate::grammar::syntax::*;
use crate::SyntacticalError;

use pest::Parser as PEG;
use std::str::FromStr;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parser;

impl FromStr for Program {
    type Err = SyntacticalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _pairs = Parser::parse(Rule::Program, s)?;

        Ok(Program)
    }
}
