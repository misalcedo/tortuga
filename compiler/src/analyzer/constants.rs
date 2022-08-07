use crate::grammar::{Expression, Number, Program, Terminal, Uri, WithoutScopeDepth};
use crate::{CompilationError, ErrorReporter};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConstantAnalysis<'a> {
    numbers: Vec<Number<'a>>,
    number_indices: HashMap<Number<'a>, usize>,
    uris: Vec<Uri<'a>>,
    uri_indices: HashMap<Uri<'a>, usize>,
}

impl<'a> From<&Program<'a>> for ConstantAnalysis<'a> {
    fn from(program: &Program<'a>) -> Self {
        let mut iterator = program.iter_post_order().without_scope_depth();
        let mut analysis = Self::default();

        for expression in iterator {
            match expression {
                Expression::Terminal(terminal) => match terminal {
                    Terminal::Number(number) if !analysis.number_indices.contains_key(number) => {
                        let index = analysis.numbers.len();

                        analysis.numbers.push(*number);
                        analysis.number_indices.insert(*number, index);
                    }
                    Terminal::Uri(uri) if !analysis.uri_indices.contains_key(uri) => {
                        let index = analysis.uris.len();

                        analysis.uris.push(*uri);
                        analysis.uri_indices.insert(*uri, index);
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }

        analysis
    }
}

impl<'a> Display for ConstantAnalysis<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "## Numbers")?;

        for (index, number) in self.numbers.iter().enumerate() {
            writeln!(f, "{}. {}", index, number)?;
        }

        writeln!(f, "## Uris")?;

        for (index, uri) in self.uris.iter().enumerate() {
            writeln!(f, "{}. {}", index, uri)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let program = Program::try_from(include_str!("../../../examples/factorial.ta")).unwrap();
        let analysis = ConstantAnalysis::from(&program);
        let numbers = vec![
            Number::positive("1"),
            Number::positive("0.5"),
            Number::positive("0"),
            Number::positive("9"),
        ];

        assert_eq!(
            numbers,
            analysis
                .numbers
                .iter()
                .copied()
                .collect::<Vec<Number<'_>>>()
        );
        assert!(analysis.uris.is_empty());
    }
}
