use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, combinator::map, sequence::delimited,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
pub enum TagPosition {
    Open,
    Close,
}

#[derive(Debug, Eq, PartialEq)]
pub struct HtmlElement {
    tag: String,
    position: TagPosition,
    attributes: Vec<HtmlAttribute>,
    children: Vec<HtmlElement>,
}

impl HtmlElement {
    pub(crate) fn new(name: &str, position: TagPosition) -> HtmlElement {
        HtmlElement {
            tag: name.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
            position,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct HtmlAttribute {
    name: String,
    value: Option<String>,
}

pub(crate) fn parse_html(input: &str) -> IResult<&str, HtmlElement> {
    delimited(tag("<"), parse_open_tag, tag(">"))(input)
}

fn parse_open_tag(input: &str) -> IResult<&str, HtmlElement> {
    map(alphanumeric1, |element| {
        HtmlElement::new(element, TagPosition::Open)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tag() {
        let actual = parse_html("<p>Hello, World!</p>");
        let expected: IResult<&str, HtmlElement> = Ok((
            "Hello, World!</p>",
            HtmlElement::new("p", TagPosition::Open),
        ));

        assert_eq!(actual, expected);
    }
}
