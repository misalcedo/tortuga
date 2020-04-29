use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending, multispace0, space0, space1};
use nom::combinator::{map_res, verify};
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;

/// Parses an input stream using the language's grammar into an abstract syntax tree.
pub fn parse(input: & str) -> Result<Vec<(&str, Vec<u64>)>, &'static str> {
    match terminated(parse_behavior, multispace0)(input) {
        Ok((remaining, result)) => Ok(result),
        Err(e) => {
            eprintln!("{}", e);
            Err("Error occurred.")
        }
    }
}

fn parse_behavior(input: &str) -> IResult<&str, Vec<(&str, Vec<u64>)>> {
    separated_list0(line_ending, parse_send_message)(input)
}

fn parse_send_message(input: &str) -> IResult<&str, (&str, Vec<u64>)> {
    delimited(
        delimited(space0, tag("("), space0),
        separated_pair(parse_reference, space1, parse_message),
        delimited(space0, tag(")"), space0)
    )(input)
}

fn parse_reference(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_message(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(space1, parse_natural_number)(input)
}

fn parse_natural_number(input: &str) -> IResult<&str, u64> {
    map_res(verify(digit1, |i: &str| i.len() == 1 || !starts_with('0', i)), |s: &str| s.parse::<u64>())(input)
}

fn starts_with(character: char, string: &str) -> bool {
    if let Some(first_character) = string.chars().next() {
        return character == first_character;
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_natural_number() {
        let text = "128";
        let expected = 128;

        assert_eq!(parse_natural_number(text), Ok(("", expected)));
    }

    #[test]
    fn parse_valid_reference() {
        let text = "add 1";
        let expected = "add";

        assert_eq!(parse_reference(text), Ok((" 1", expected)));
    }

    #[test]
    fn parse_valid_message() {
        let text = "1 4 0 )";
        let expected = vec![1, 4, 0];

        assert_eq!(parse_message(text), Ok((" )", expected)));
    }

    #[test]
    fn parse_valid_send_message() {
        let text = "( add 1 4 0 )";
        let expected = ("add", vec![1, 4, 0]);

        assert_eq!(parse_send_message(text), Ok(("", expected)));
    }

    #[test]
    fn parse_valid_behavior() {
        let text = "( add 1 4 0 )\n(substract 5 3 )   \r\n   ( multiply 2 2)  \r\n ";
        let expected = vec![
            ("add", vec![1, 4, 0]),
            ("substract", vec![5, 3]),
            ("multiply", vec![2, 2]),
        ];

        assert_eq!(parse_behavior(text), Ok(("", expected)));
    }
}