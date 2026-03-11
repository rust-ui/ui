use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::char;
use nom::combinator::{opt, recognize};
use nom::sequence::{delimited, pair};

use crate::ast::take_until_unbalanced;

pub mod length {
    use super::*;

    pub fn parse(input: &str) -> IResult<&str, &str> {
        alt((tag("0"), number, functional_notation))(input)
    }

    // Parser for numeric values
    fn number(input: &str) -> IResult<&str, &str> {
        recognize(pair(pair(opt(char('-')), take_while1(|c: char| c.is_ascii_digit() || c == '.')), opt(unit)))(input)
    }

    // Parser for units like px, em, rem, etc.
    fn unit(input: &str) -> IResult<&str, &str> {
        let parser1 = alt((
            tag("%"),
            tag("px"),
            tag("em"),
            tag("rem"),
            tag("vh"),
            tag("vw"),
            tag("vmin"),
            tag("vmax"),
            tag("pt"),
            tag("pc"),
            tag("in"),
            tag("cm"),
        ));

        let parser2 = alt((
            tag("mm"),
            tag("cap"),
            tag("ch"),
            tag("ex"),
            tag("lh"),
            tag("rlh"),
            tag("vw"),
            tag("vh"),
            tag("vi"),
            tag("vb"),
            tag("vmin"),
            tag("vmax"),
            tag("q"),
        ));

        alt((parser1, parser2))(input)
    }

    fn functional_notation(input: &str) -> IResult<&str, &str> {
        alt((
            delimited(tag("calc("), take_until_unbalanced('(', ')'), char(')')),
            delimited(tag("min("), take_until_unbalanced('(', ')'), char(')')),
            delimited(tag("max("), take_until_unbalanced('(', ')'), char(')')),
            delimited(tag("clamp("), take_until_unbalanced('(', ')'), char(')')),
        ))(input)
    }

    #[test]
    fn len() {
        let input = "calc(theme(fontSize.4xl)/1.125)";
        let (_, result) = parse(input).unwrap();
        assert_eq!(result, "theme(fontSize.4xl)/1.125");
    }

    #[test]
    fn num_len() {
        let input = "1";
        assert!('1'.is_ascii_digit());
        let result = number(input).unwrap();
        assert_eq!(result, ("", "1"));
    }
}

pub mod arbitrary {
    use super::*;

    pub fn parse(input: &str) -> IResult<&str, (Option<&str>, &str)> {
        parse_with_property(input).or_else(|_| {
            let (input, val) = value(input)?;
            Ok((input, (None, val)))
        })
    }

    fn parse_with_property(input: &str) -> IResult<&str, (Option<&str>, &str)> {
        let (input, prop) = opt(property)(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, val) = value(input)?;
        Ok((input, (prop, val)))
    }

    fn property(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_ascii_lowercase() || c == '-')(input)
    }

    fn value(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c != '\n')(input)
    }

    #[test]
    fn test_arb() {
        let result = parse("length:theme(someScale.someValue)").unwrap();
        assert_eq!(result.1, (Some("length"), "theme(someScale.someValue)"));
    }
}

pub mod color {
    use super::*;

    pub fn parse(input: &str) -> IResult<&str, (&str, &str)> {
        let (input, func) = color_function(input)?;
        let (input, args) = delimited(char('('), arguments, char(')'))(input)?;
        Ok((input, (func, args)))
    }

    fn color_function(input: &str) -> IResult<&str, &str> {
        alt((
            tag("rgba"),
            tag("rgb"),
            tag("hsla"),
            tag("hsl"),
            tag("hwb"),
            tag("oklab"),
            tag("oklch"),
            tag("lab"),
            tag("lch"),
        ))(input)
    }

    fn arguments(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c != ')')(input)
    }

    #[test]
    fn not_len() {
        let input = "calc(theme(fontSize.4xl)/1.125)";
        let result = parse(input);

        assert!(result.is_err(), "Should not be a length");
    }
}

pub mod image {
    use super::*;

    pub fn parse(input: &str) -> IResult<&str, (&str, &str)> {
        let (input, func) = image_function(input)?;
        let (input, args) = delimited(char('('), arguments, char(')'))(input)?;
        Ok((input, (func, args)))
    }

    fn image_function(input: &str) -> IResult<&str, &str> {
        alt((
            tag("url"),
            tag("image"),
            tag("image-set"),
            tag("cross-fade"),
            tag("element"),
            tag("repeating-linear-gradient"),
            tag("repeating-radial-gradient"),
            tag("repeating-conic-gradient"),
            tag("linear-gradient"),
            tag("radial-gradient"),
            tag("conic-gradient"),
        ))(input)
    }

    fn arguments(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c != ')')(input)
    }
}

pub fn parse_single_digit_decimal(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    let mut dot_index = None;
    for (i, c) in input.char_indices() {
        if c == '.' {
            if dot_index.is_some() || i == 0 || i == input.len() - 1 || input.len() - i != 2 {
                return false;
            }
            dot_index = Some(i);
        } else if !c.is_ascii_digit() {
            return false;
        }
    }

    true
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

#[test]
fn test_valid_single_digit_decimal() {
    assert!(parse_single_digit_decimal("1.5"));
    assert!(parse_single_digit_decimal("11.5"));
    assert!(parse_single_digit_decimal("0.0"));
    assert!(parse_single_digit_decimal("9.9"));
    assert!(parse_single_digit_decimal("123"));
    assert!(parse_single_digit_decimal("0"));
    assert!(parse_single_digit_decimal("1"));
    assert!(parse_single_digit_decimal("12345"));
    assert!(parse_single_digit_decimal("9.0"));
    assert!(parse_single_digit_decimal("0.9"));
}

#[test]
fn test_invalid_single_digit_decimal() {
    assert!(!parse_single_digit_decimal(""));
    assert!(!parse_single_digit_decimal("1.55"));
    assert!(!parse_single_digit_decimal("11.55"));
    assert!(!parse_single_digit_decimal("1."));
    assert!(!parse_single_digit_decimal(".5"));
    assert!(!parse_single_digit_decimal("1.5.5"));
    assert!(!parse_single_digit_decimal("a.5"));
    assert!(!parse_single_digit_decimal("1.a"));
    assert!(!parse_single_digit_decimal("1 .5"));
    assert!(!parse_single_digit_decimal("1. 5"));
    assert!(!parse_single_digit_decimal("-1.5"));
    assert!(!parse_single_digit_decimal("+1.5"));
    assert!(!parse_single_digit_decimal("1.5e3"));
    assert!(!parse_single_digit_decimal("1.5E3"));
    assert!(!parse_single_digit_decimal("1.5f"));
    assert!(!parse_single_digit_decimal("1.5d"));
    assert!(!parse_single_digit_decimal("1,5"));
    assert!(!parse_single_digit_decimal("1..5"));
}
