//! Pretty print Tortuga [`Program`]s and errors.

use crate::compiler::{Lexeme, Token};
use crate::grammar::*;
use crate::{runtime, Kind, LexicalError, SyntacticalError};
use colored::*;
use std::fmt::Display;
use std::io::{self, Write};

/// A printer to standard out for Tortuga programs.
pub struct PrettyPrinter<StdOut: Write, StdErr: Write> {
    std_out: StdOut,
    std_err: StdErr,
    nesting: usize,
    spaces: usize,
}

fn print_token_to<W: Write>(token: Token<'_>, mut write: W) -> io::Result<()> {
    let kind = token.kind().to_string();
    let lexeme = token.as_str().to_string();
    let start = token.lexeme().start().to_string();

    match token.kind() {
        Kind::Number => writeln!(
            write,
            "[{}] \"{}\" = {} {} {}",
            kind.green().bold(),
            lexeme.blue(),
            lexeme
                .parse::<runtime::Number>()
                .unwrap_or_default()
                .to_string()
                .blue()
                .bold(),
            "@".yellow().bold(),
            start.red()
        )?,
        Kind::Identifier => writeln!(
            write,
            "[{}] \"{}\" {} {}",
            kind.green().bold(),
            lexeme.blue(),
            "@".yellow().bold(),
            start.red()
        )?,
        _ => writeln!(
            write,
            "[{}] {} {}",
            kind.green().bold(),
            "@".yellow().bold(),
            start.red()
        )?,
    }

    Ok(())
}

impl<StdOut: Write, StdErr: Write> PrettyPrinter<StdOut, StdErr> {
    /// Create a new pretty printer.
    pub fn new(std_out: StdOut, std_err: StdErr) -> Self {
        PrettyPrinter {
            std_out,
            std_err,
            nesting: 0,
            spaces: 2,
        }
    }

    fn decrement_nesting(&mut self) {
        self.nesting -= self.spaces;
    }

    fn increment_nesting(&mut self) -> io::Result<()> {
        self.nesting += self.spaces;

        self.print_nesting(' ')
    }

    fn print_nesting<D: Display>(&mut self, value: D) -> io::Result<()> {
        write!(self.std_out, "{:>1$}", value, self.nesting)
    }

    /// Prints a [`Display`] instance to this [`PrettyPrinter`]'s `std_out` [`Write`]r.
    pub fn print<D: Display>(&mut self, value: D) -> io::Result<()> {
        write!(self.std_out, "{}", value)
    }

    /// Prints a [`Display`] instance to this [`PrettyPrinter`]'s `std_err` [`Write`]r.
    pub fn print_err<D: Display>(&mut self, value: D) -> io::Result<()> {
        write!(self.std_err, "{}", value)
    }

    /// Prints a [`Token`] to this [`PrettyPrinter`]'s `std_out` [`Write`]r.
    pub fn print_token(&mut self, token: Token) -> io::Result<()> {
        print_token_to(token, &mut self.std_out)
    }

    /// Prints a [`SyntacticalError`] to this [`PrettyPrinter`]'s `std_err` [`Write`]r.
    pub fn print_syntactical_error(&mut self, error: SyntacticalError) -> io::Result<()> {
        match error {
            SyntacticalError::Incomplete => {
                self.print_error_prefix("EOF")?;
                writeln!(self.std_err, "{error}")
            }
            SyntacticalError::Multiple => {
                self.print_error_prefix("Multiple")?;
                writeln!(self.std_err, "{error}")
            }
            SyntacticalError::NoMatch(token) => {
                self.print_error_prefix("NoMatch")?;
                write!(self.std_err, "No grammar rule matched the token: ")?;
                print_token_to(
                    Token::new(Lexeme::new(*token.start(), token.as_str()), *token.kind()),
                    &mut self.std_err,
                )
            }
            SyntacticalError::Lexical(errors) => {
                for error in errors {
                    self.print_lexical_error(error)?;
                }

                Ok(())
            }
        }
    }

    /// Prints a [`LexicalError`] to this [`PrettyPrinter`]'s `std_err` [`Write`]r.
    pub fn print_lexical_error(&mut self, error: LexicalError) -> io::Result<()> {
        let kind = error.kind();
        let lexeme = error.as_str().to_string();
        let start = error.start().to_string();

        self.print_error_prefix(kind)?;

        writeln!(
            self.std_err,
            "\"{}\" {} {}",
            lexeme.blue(),
            "@".yellow().bold(),
            start.red()
        )
    }

    fn print_error_prefix<T: ToString>(&mut self, kind: T) -> io::Result<()> {
        write!(
            self.std_err,
            "[{}|{}] ",
            "ERROR".red().bold(),
            kind.to_string().green().bold()
        )
    }

    /// Prints a [`Program`] to this [`PrettyPrinter`]'s `std_out` [`Write`]r.
    pub fn print_program(&mut self, program: &Program) -> io::Result<()> {
        match program {
            Program::Expressions(expressions) => self.print_expressions(expressions),
            Program::Comparisons(comparisons) => self.print_comparisons(comparisons),
        }
    }

    fn print_expressions(&mut self, expressions: &Expressions) -> io::Result<()> {
        self.print_expression(expressions.head())?;
        writeln!(self.std_out)?;

        for expression in expressions.tail() {
            self.print_expression(expression)?;
            writeln!(self.std_out)?;
        }

        Ok(())
    }

    fn print_comparisons(&mut self, comparisons: &Comparisons) -> io::Result<()> {
        self.print_expression(comparisons.lhs())?;
        self.print_comparison(comparisons.comparisons().head())?;

        for comparison in comparisons.comparisons().tail() {
            self.print_comparison(comparison)?;
        }

        Ok(())
    }

    fn print_expression(&mut self, expression: &Expression) -> io::Result<()> {
        match expression {
            Expression::Assignment(assignment) => self.print_assignment(assignment)?,
            Expression::Call(call) => self.print_call(call)?,
            Expression::Operation(operation) => self.print_operation(operation)?,
            Expression::Grouping(grouping) => self.print_grouping(grouping)?,
            Expression::Identifier(identifier) => self.print_identifier(identifier)?,
            Expression::Number(number) => self.print_number(number)?,
        }

        Ok(())
    }

    fn print_comparison(&mut self, comparison: &Comparison) -> io::Result<()> {
        self.print_comparator(comparison.comparator())?;
        self.print_expression(comparison.rhs())
    }

    fn print_comparator(&mut self, comparator: &Comparator) -> io::Result<()> {
        write!(self.std_out, " {} ", comparator)
    }

    fn print_assignment(&mut self, assignment: &Assignment) -> io::Result<()> {
        self.print_function(assignment.function())?;
        write!(self.std_out, " = ")?;
        self.print_block(assignment.block())
    }

    fn print_function(&mut self, function: &Function) -> io::Result<()> {
        self.print_name(function.name())?;
        self.print_parameters(function.parameters())?;

        Ok(())
    }

    fn print_name(&mut self, name: &Name) -> io::Result<()> {
        match name {
            Name::Anonymous => write!(self.std_out, "_"),
            Name::Identified(identifier) => {
                write!(self.std_out, "@")?;
                self.print_identifier(identifier)
            }
        }
    }

    fn print_identifier(&mut self, identifier: &lexical::Identifier) -> io::Result<()> {
        write!(self.std_out, "{}", identifier.as_str())
    }

    fn print_parameters(&mut self, parameters: &[Pattern]) -> io::Result<()> {
        if parameters.is_empty() {
            return Ok(());
        }

        write!(self.std_out, "(")?;

        let mut iterator = parameters.iter().peekable();

        while let Some(parameter) = iterator.next() {
            self.print_pattern(parameter)?;

            if iterator.peek().is_some() {
                write!(self.std_out, ", ")?;
            }
        }

        write!(self.std_out, ")")
    }

    fn print_pattern(&mut self, pattern: &Pattern) -> io::Result<()> {
        match pattern {
            Pattern::Function(function) => self.print_function(function),
            Pattern::Refinement(refinement) => self.print_refinement(refinement),
            Pattern::Bounds(bounds) => self.print_bounds(bounds),
        }
    }

    fn print_refinement(&mut self, refinement: &Refinement) -> io::Result<()> {
        self.print_name(refinement.name())?;
        self.print_comparator(refinement.comparator())?;
        self.print_expression(refinement.constraint())
    }

    fn print_bounds(&mut self, bounds: &Bounds) -> io::Result<()> {
        self.print_expression(bounds.left().constraint())?;
        self.print_inequality(bounds.left().inequality())?;
        self.print_name(bounds.name())?;
        self.print_inequality(bounds.right().inequality())?;
        self.print_expression(bounds.right().constraint())
    }

    fn print_inequality(&mut self, inequality: &Inequality) -> io::Result<()> {
        write!(self.std_out, " {} ", inequality)
    }

    fn print_block(&mut self, block: &Block) -> io::Result<()> {
        if block.tail().is_empty() {
            self.print_expression(block.head())
        } else {
            writeln!(self.std_out, "{:>1$}", '[', self.nesting)?;

            self.increment_nesting()?;

            self.print_expression(block.head())?;

            for expression in block.tail() {
                writeln!(self.std_out)?;
                self.print_nesting(' ')?;
                self.print_expression(expression)?;
            }

            writeln!(self.std_out)?;

            self.decrement_nesting();

            write!(self.std_out, "{:>1$}", ']', self.nesting)
        }
    }

    fn print_operation(&mut self, operation: &Operation) -> io::Result<()> {
        self.print_expression(operation.lhs())?;

        let operator = match operation.operator() {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Exponent => "^",
            Operator::Modulo => "%",
            Operator::Tolerance => "~",
        };

        write!(self.std_out, " {operator} ")?;

        self.print_expression(operation.rhs())
    }

    fn print_number(&mut self, number: &Number) -> io::Result<()> {
        if number.is_negative() {
            write!(self.std_out, "-")?;
        }

        let lexeme = number.number().as_str();
        let value: runtime::Number = lexeme
            .parse()
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

        write!(self.std_out, "{}", value)
    }

    fn print_call(&mut self, call: &Call) -> io::Result<()> {
        self.print_expression(call.callee())?;
        self.print_arguments(call.arguments())
    }

    fn print_arguments(&mut self, arguments: &Arguments) -> io::Result<()> {
        write!(self.std_out, "(")?;

        self.print_expression(arguments.head())?;

        for argument in arguments.tail() {
            write!(self.std_out, ", ")?;
            self.print_expression(argument)?;
        }

        write!(self.std_out, ")")
    }

    fn print_grouping(&mut self, grouping: &Grouping) -> io::Result<()> {
        write!(self.std_out, "(")?;
        self.print_expression(grouping.inner())?;
        write!(self.std_out, ")")
    }
}
