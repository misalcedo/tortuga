//! Uses a PEG grammar to validate a source file.

use crate::CommandLineError;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "../bin/grammar.pest"]
pub struct TortugaParser;

///
pub fn validate_file(source: &str) -> Result<(), CommandLineError> {
    let pairs = TortugaParser::parse(Rule::Program, source)?;

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::IDENTIFIER => println!("Letter:  {}", inner_pair.as_str()),
                Rule::Number => println!("Digit:   {}", inner_pair.as_str()),
                _ => (),
            };
        }
    }

    Ok(())
}
