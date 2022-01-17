//! Parse a sequence of tokens into a syntax tree.

mod tokens;

use crate::compiler::parser::tokens::TokenMatcher;
use crate::compiler::{Kind, Token};
use crate::grammar::lexical;
use crate::grammar::syntax::*;
use crate::{Scanner, SyntacticalError};
use std::str::FromStr;
use tokens::Tokens;

const COMPARISON_KINDS: &[Kind] = &[
    Kind::LessThan,
    Kind::GreaterThan,
    Kind::LessThanOrEqualTo,
    Kind::GreaterThanOrEqualTo,
    Kind::Equal,
    Kind::NotEqual,
];
const NAME_KINDS: &[Kind] = &[Kind::At, Kind::Underscore];
const INEQUALITY_KINDS: &[Kind] = &[
    Kind::LessThan,
    Kind::GreaterThan,
    Kind::LessThanOrEqualTo,
    Kind::GreaterThanOrEqualTo,
];

/// A recursive descent LL(1) parser for the syntax grammar.
/// Parses a sequence of `Token`s into syntax tree.
///
/// # Examples
/// ```rust
/// use tortuga::Program;
///
/// assert!("(2 + 2#10) ^ 2 = 16".parse::<Program>().is_ok());
/// ```
pub struct Parser<'a, T: Tokens<'a>> {
    tokens: &'a mut T,
}

impl<'a, T: Tokens<'a>> Parser<'a, T> {
    /// Advances the token sequence and returns the next value if the token is one of the expected [`Kind`]s.
    ///
    /// Returns [`Err`] when at the end of the sequence,
    /// if the token's kind does not match, or if the token is invalid.
    fn next_kind<Matcher: TokenMatcher>(
        &mut self,
        matcher: Matcher,
    ) -> Result<Token<'a>, SyntacticalError> {
        match self.tokens.next_matches(matcher) {
            Some(true) => self.tokens.next_token(),
            Some(false) => Err(SyntacticalError::NoMatch(
                self.tokens
                    .peek_token()
                    .copied()
                    .ok_or(SyntacticalError::Incomplete)?
                    .into(),
            )),
            None => Err(SyntacticalError::Incomplete),
        }
    }

    /// Generate a syntax tree rooted at a `Program` for this `Parser`'s sequence of tokens.
    pub fn parse(mut self) -> Result<Program, SyntacticalError> {
        let expression = self.parse_expression()?;

        match self.tokens.peek_kind() {
            Some(
                Kind::LessThan
                | Kind::GreaterThan
                | Kind::LessThanOrEqualTo
                | Kind::GreaterThanOrEqualTo
                | Kind::Equal
                | Kind::NotEqual,
            ) => self.parse_comparisons(expression),
            _ => self.parse_expressions(expression),
        }
    }

    fn parse_expressions(&mut self, expression: Expression) -> Result<Program, SyntacticalError> {
        let mut expressions = Vec::new();

        while self.tokens.has_next() {
            expressions.push(self.parse_expression()?);
        }

        Ok(List::new(expression, expressions).into())
    }

    fn parse_comparisons(&mut self, expression: Expression) -> Result<Program, SyntacticalError> {
        let head = self.parse_comparison()?;
        let mut comparisons = Vec::new();

        while self.tokens.has_next() {
            comparisons.push(self.parse_comparison()?);
        }

        Ok(Comparisons::new(expression, List::new(head, comparisons)).into())
    }

    fn parse_comparison(&mut self) -> Result<Comparison, SyntacticalError> {
        let operator = self.parse_comparator()?;
        let expression = self.parse_expression()?;

        Ok(Comparison::new(operator, expression))
    }

    fn parse_comparator(&mut self) -> Result<Comparator, SyntacticalError> {
        let operator = match self.next_kind(COMPARISON_KINDS)?.kind() {
            Kind::LessThan => Comparator::LessThan,
            Kind::GreaterThan => Comparator::GreaterThan,
            Kind::LessThanOrEqualTo => Comparator::LessThanOrEqualTo,
            Kind::GreaterThanOrEqualTo => Comparator::GreaterThanOrEqualTo,
            Kind::NotEqual => Comparator::NotEqualTo,
            _ => Comparator::EqualTo,
        };

        Ok(operator)
    }

    fn parse_expression(&mut self) -> Result<Expression, SyntacticalError> {
        if let Some(true) = self.tokens.next_matches(NAME_KINDS) {
            self.parse_assignment().map(Expression::from)
        } else {
            self.parse_arithmetic().map(Expression::from)
        }
    }

    fn parse_arithmetic(&mut self) -> Result<Arithmetic, SyntacticalError> {
        Ok(self.parse_epsilon()?.into())
    }

    fn parse_epsilon(&mut self) -> Result<Epsilon, SyntacticalError> {
        let lhs = self.parse_modulo()?;
        let mut rhs = None;

        if self.tokens.next_if_match(Kind::Tilde).is_some() {
            rhs = Some(self.parse_modulo()?);
        }

        Ok(Epsilon::new(lhs, rhs))
    }

    fn parse_modulo(&mut self) -> Result<Modulo, SyntacticalError> {
        let head = self.parse_sum()?;
        let mut tail = Vec::new();

        while self.tokens.next_if_match(Kind::Percent).is_some() {
            tail.push(self.parse_sum()?);
        }

        Ok(List::new(head, tail))
    }

    fn parse_sum(&mut self) -> Result<Sum, SyntacticalError> {
        let head = self.parse_product()?;
        let mut tail = Vec::new();

        while let Some(token) = self.tokens.next_if_match(&[Kind::Plus, Kind::Minus]) {
            let rhs = self.parse_product()?;
            let operation = match token.kind() {
                Kind::Plus => AddOrSubtract::Add(rhs),
                _ => AddOrSubtract::Subtract(rhs),
            };

            tail.push(operation);
        }

        Ok(List::new(head, tail))
    }

    fn parse_product(&mut self) -> Result<Product, SyntacticalError> {
        let head = self.parse_power()?;
        let mut tail = Vec::new();

        while let Some(token) = self.tokens.next_if_match(&[Kind::Star, Kind::Slash]) {
            let rhs = self.parse_power()?;
            let operation = match token.kind() {
                Kind::Star => MultiplyOrDivide::Multiply(rhs),
                _ => MultiplyOrDivide::Divide(rhs),
            };

            tail.push(operation);
        }

        Ok(List::new(head, tail))
    }

    fn parse_power(&mut self) -> Result<Power, SyntacticalError> {
        let lhs = self.parse_call()?;
        let mut rhs = Vec::new();

        while self.tokens.next_if_match(Kind::Caret).is_some() {
            rhs.push(self.parse_call()?);
        }

        Ok(List::new(lhs, rhs))
    }

    fn parse_call(&mut self) -> Result<Call, SyntacticalError> {
        let callee = self.parse_primary()?;

        let mut arguments = Vec::new();

        while let Some(true) = self.tokens.next_matches(Kind::LeftParenthesis) {
            arguments.push(self.parse_arguments()?);
        }

        Ok(Call::new(callee, arguments))
    }

    fn parse_primary(&mut self) -> Result<Primary, SyntacticalError> {
        let token = self.next_kind(&[
            Kind::Minus,
            Kind::Number,
            Kind::Identifier,
            Kind::LeftParenthesis,
        ])?;

        match token.kind() {
            Kind::Minus | Kind::Number => self.parse_number(token).map(Primary::from),
            Kind::Identifier => self.parse_identifier(token).map(Primary::from),
            _ => self.parse_grouping(token).map(Primary::from),
        }
    }

    fn parse_number(&mut self, token: Token) -> Result<Number, SyntacticalError> {
        match token.kind() {
            Kind::Minus => {
                let number = self.next_kind(Kind::Number)?;

                Ok(Number::new(
                    true,
                    lexical::Number::new(number.as_str()),
                ))
            }
            _ => Ok(Number::new(
                false,
                lexical::Number::new(token.as_str()),
            )),
        }
    }

    fn parse_identifier(
        &mut self,
        identifier: Token,
    ) -> Result<lexical::Identifier, SyntacticalError> {
        Ok(lexical::Identifier::new(identifier.as_str()))
    }

    fn parse_arguments(&mut self) -> Result<Arguments, SyntacticalError> {
        self.next_kind(Kind::LeftParenthesis)?;

        let head = self.parse_expression()?;
        let mut tail = Vec::new();

        while self.tokens.next_if_match(Kind::Comma).is_some() {
            tail.push(self.parse_expression()?);
        }

        self.next_kind(Kind::RightParenthesis)?;

        Ok(List::new(head, tail))
    }

    fn parse_grouping(&mut self, _: Token) -> Result<Grouping, SyntacticalError> {
        let expression = self.parse_expression()?;

        self.next_kind(Kind::RightParenthesis)?;

        Ok(expression.into())
    }

    fn parse_assignment(&mut self) -> Result<Assignment, SyntacticalError> {
        let function = self.parse_function()?;

        self.next_kind(Kind::Equal)?;

        let block = self.parse_block()?;

        Ok(Assignment::new(function, block))
    }

    fn parse_function(&mut self) -> Result<Function, SyntacticalError> {
        let name = self.parse_name()?;
        let parameters = self.parse_parameters()?;

        Ok(Function::new(name, parameters))
    }

    fn parse_name(&mut self) -> Result<Name, SyntacticalError> {
        let token = self.next_kind(NAME_KINDS)?;

        match token.kind() {
            Kind::At => {
                let identifier = self.next_kind(Kind::Identifier)?;

                Ok(Name::from(lexical::Identifier::new(
                    identifier.as_str()
                )))
            }
            _ => Ok(Name::Anonymous),
        }
    }

    fn parse_parameters(&mut self) -> Result<Vec<Pattern>, SyntacticalError> {
        let mut parameters = Vec::new();

        if self.tokens.next_if_match(Kind::LeftParenthesis).is_some() {
            parameters.push(self.parse_pattern()?);

            while self.tokens.next_if_match(Kind::Comma).is_some() {
                parameters.push(self.parse_pattern()?);
            }

            self.next_kind(Kind::RightParenthesis)?;
        }

        Ok(parameters)
    }

    fn parse_pattern(&mut self) -> Result<Pattern, SyntacticalError> {
        if let Some(true) = self.tokens.next_matches(NAME_KINDS) {
            let name = self.parse_name()?;

            if let Some(true) = self.tokens.next_matches(COMPARISON_KINDS) {
                self.parse_refinement(name)
            } else {
                let parameters = self.parse_parameters()?;

                Ok(Function::new(name, parameters).into())
            }
        } else {
            Ok(self.parse_bounds()?.into())
        }
    }

    fn parse_inequality(&mut self) -> Result<Inequality, SyntacticalError> {
        let operator = match self.next_kind(INEQUALITY_KINDS)?.kind() {
            Kind::LessThan => Inequality::LessThan,
            Kind::GreaterThan => Inequality::GreaterThan,
            Kind::LessThanOrEqualTo => Inequality::LessThanOrEqualTo,
            _ => Inequality::GreaterThanOrEqualTo,
        };

        Ok(operator)
    }

    fn parse_bounds(&mut self) -> Result<Bounds, SyntacticalError> {
        let left = Bound::new(self.parse_arithmetic()?, self.parse_inequality()?);

        let name = self.parse_name()?;

        let right_inequality = self.parse_inequality()?;
        let right_constraint = self.parse_arithmetic()?;
        let right = Bound::new(right_constraint, right_inequality);

        Ok(Bounds::new(left, name, right))
    }

    fn parse_refinement(&mut self, name: Name) -> Result<Pattern, SyntacticalError> {
        let comparator = self.parse_comparator()?;
        let arithmetic = self.parse_arithmetic()?;

        Ok(Refinement::new(name, comparator, arithmetic).into())
    }

    fn parse_block(&mut self) -> Result<Block, SyntacticalError> {
        if let Some(Kind::LeftBracket) = self.tokens.peek_kind() {
            self.next_kind(Kind::LeftBracket)?;

            let head = self.parse_expression()?;
            let mut tail = vec![self.parse_expression()?];

            while let Some(false) = self.tokens.next_matches(Kind::RightBracket) {
                tail.push(self.parse_expression()?);
            }

            self.next_kind(Kind::RightBracket)?;

            Ok(List::new(head, tail))
        } else {
            let head = self.parse_expression()?;

            Ok(List::new(head, Vec::new()))
        }
    }
}

impl FromStr for Program {
    type Err = SyntacticalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut scanner = Scanner::from(s).peekable();
        let parser = Parser {
            tokens: &mut scanner,
        };

        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert!("2".parse::<Program>().is_ok());
    }

    #[test]
    fn parse_negative_number() {
        assert!("-2#100".parse::<Program>().is_ok());
    }

    #[test]
    fn parse_identifier() {
        assert!("xyz".parse::<Program>().is_ok());
    }

    #[test]
    fn parse_example() {
        assert!(include_str!("../../../examples/example.ta")
            .parse::<Program>()
            .is_ok())
    }

    #[test]
    fn parse_factorial() {
        assert!(include_str!("../../../examples/factorial.ta")
            .parse::<Program>()
            .is_ok())
    }

    #[test]
    fn parse_bad() {
        assert!(include_str!("../../../examples/bad.ta")
            .parse::<Program>()
            .is_err())
    }
}
