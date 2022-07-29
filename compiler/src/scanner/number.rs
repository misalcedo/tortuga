//! Lexical representation of numbers within Tortuga.
//! Numbers in Tortuga cannot have leading 0s in the integer portion
//! and cannot have trailing 0s in the fraction portion.

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_number<I: Into<Number>>(number: &str, value: I) {
        assert_eq!(number.parse::<Number>(), Ok(value.into()));
    }

    fn parse_number() {
        validate_number("0", 0);
        validate_number("0.0", 0);
        validate_number(".0", 0);
        validate_number("0.", 0);
        validate_number("2", 2);
        validate_number("4", 4);
        validate_number("21", 21);
        validate_number("100", 100);
        validate_number(".1", 0.1);
        validate_number(".5", 0.5);
        validate_number("1.0", 1.0);
        validate_number("4.5", 4.5);
        validate_number("0.5", 0.5);
        validate_number("10000.5002", 10000.5002);
        validate_number("7.002", 7.002);

        validate_number("2#0", 0);
        validate_number("16#F", 15);
        validate_number("3#21", 7);
        validate_number("2#100", 4);
        validate_number("2#.1", 0.5);
        validate_number("10#.5", 0.5);
        validate_number("12#1.0", 1.0);
        validate_number("20#4.5", 4.25);
        validate_number("30#0.5", 0.16666666666666666);
        validate_number("36#10000.5002", 1679616.1388900797);
        validate_number("32#7.002", 7.00006103515625);
    }

    fn invalidate_number(number: &str) {
        assert_eq!(
            number.parse::<Number>(),
            Err(ParseNumberError::from(number))
        );
    }

    fn parse_invalid_number() {
        invalidate_number(".");
        invalidate_number("20#.");
        invalidate_number("008#1.0");
        invalidate_number("0#1.0");
        invalidate_number("0008");
        invalidate_number(".1000");
        invalidate_number("2#.100");
        invalidate_number("37#1.0");
        invalidate_number("2#4.0");
        invalidate_number("#1.0");
        invalidate_number("#.");
    }
}
