mod ast;

/// Parses an input stream using the language's grammar into an abstract syntax tree.
pub fn parse() -> ast::AbstractSyntaxTree {
    println!("Hi!");
    ast::AbstractSyntaxTree {}
}

fn parse_behavior() {

}

fn parse_transmit() {

}

fn parse_reference() {
}

fn parse_natural_number() {
}

fn parse_

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_natural_number() {
        let text = "".as_bytes();
        let expected =
            Header::version_1(([255, 255, 255, 255], [255, 255, 255, 255], 65535, 65535).into());

        assert_eq!(parse_v1_header(text), Ok((&[][..], expected)));
    }
}