use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};

use super::{ASTVariant, AstElements, AstParseOptions, AstStyle};

pub fn parse_tailwind<'a>(class: &[&'a str], options: AstParseOptions<'a>) -> Vec<Result<AstStyle<'a>, &'a str>> {
    class
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(|c| match parse_style(c, &options) {
            Ok(("", style)) => Ok(style),
            _ => Err(c),
        })
        .collect()
}

#[inline]
fn parse_style<'a>(input: &'a str, options: &AstParseOptions<'a>) -> IResult<&'a str, AstStyle<'a>> {
    // v4 supports ! at end (flex!), v3 supports ! at start (!flex)
    let (rest, (variants, important_prefix, negative, elements, arbitrary, important_suffix)) = tuple((
        many0(|s| parse_variant(options.separator, s)),
        opt(char('!')),
        opt(char('-')),
        opt(|s| parse_elements(options.prefix, s)),
        opt(parse_arbitrary),
        opt(char('!')), // v4: important at end
    ))(input)?;

    let source = &input[..input.len() - rest.len()];

    let variants = variants
        .into_iter()
        .map(|v| match v {
            ASTVariant::Normal(v) => v,
            ASTVariant::DataAttribute(v) => v,
            ASTVariant::ArbitraryAttribute(v) => v,
        })
        .collect();

    Ok((
        rest,
        AstStyle {
            source,
            important: important_prefix.is_some() || important_suffix.is_some(),
            negative: negative.is_some(),
            variants,
            elements: elements.unwrap_or_default().elements,
            arbitrary,
        },
    ))
}

#[inline]
fn parse_elements<'a>(prefix: &'a str, input: &'a str) -> IResult<&'a str, AstElements<'a>> {
    #[inline]
    fn parse_head(input: &str) -> IResult<&str, &str> {
        let stop = |c: char| -> bool {
            // space, and ! for v4 important suffix
            matches!(c, ' ' | '\n' | '\r' | '-' | '[' | ']' | '(' | ')' | '!')
        };
        take_till1(stop)(input)
    }
    #[inline]
    fn parse_rest(input: &str) -> IResult<&str, &str> {
        let (rest, (_, out)) = tuple((char('-'), parse_head))(input)?;
        Ok((rest, out))
    }

    let (rest, (_, first, other)) = tuple((tag(prefix), parse_head, many0(parse_rest)))(input)?;
    let mut out = vec![first];
    out.extend(other);
    Ok((rest, AstElements { elements: out }))
}

#[inline]
pub fn parse_variant<'a>(separator: &'a str, input: &'a str) -> IResult<&'a str, ASTVariant<'a>> {
    let parser = alt((parse_data_attribute_variant, parse_arbitrary_attribute_variant, parse_normal_variant));

    let (rest, (v, _)) = tuple((parser, tag(separator)))(input)?;
    Ok((rest, v))
}

// https://tailwindcss.com/docs/hover-focus-and-other-states#using-arbitrary-variants
#[inline]
fn parse_normal_variant(input: &str) -> IResult<&str, ASTVariant<'_>> {
    // Include '/' for named group variants like group-hover/dropdown, peer-checked/label
    // Include '*' for child selector variant (Tailwind 3.4+) like *:text-gray-500
    // Include '@' for container query variants (Tailwind v4) like @lg:, @sm:, @container:, @container/sidebar:
    // https://tailwindcss.com/docs/responsive-design#container-queries
    let parser = take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '/' || c == '*' || c == '@');
    let (rest, result) = parser(input)?;
    Ok((rest, ASTVariant::Normal(result)))
}

// https://tailwindcss.com/docs/hover-focus-and-other-states#data-attributes
// https://tailwindcss.com/docs/hover-focus-and-other-states#supports-rules
#[inline]
pub fn parse_data_attribute_variant(input: &str) -> IResult<&str, ASTVariant<'_>> {
    let tag_prefix = alt((tag("data-"), tag("supports-")));
    let mut parser = delimited(tag_prefix, take_till1(|c| c == ']'), tag("]"));
    let (rest, _) = parser(input)?;
    let entire_variant = &input[..input.len() - rest.len()];
    Ok((rest, ASTVariant::DataAttribute(entire_variant)))
}

// https://tailwindcss.com/docs/hover-focus-and-other-states#using-arbitrary-variants
#[inline]
pub fn parse_arbitrary_attribute_variant(input: &str) -> IResult<&str, ASTVariant<'_>> {
    let mut parser = delimited(tag("["), take_until_unbalanced('[', ']'), tag("]"));
    let (rest, _) = parser(input)?;
    let entire_variant = &input[..input.len() - rest.len()];
    Ok((rest, ASTVariant::ArbitraryAttribute(entire_variant)))
}

#[inline]
fn parse_arbitrary(input: &str) -> IResult<&str, &str> {
    let parser = delimited(tag("["), take_until_unbalanced('[', ']'), tag("]"));
    let (rest, (_, arbitrary)) = tuple((opt(char('-')), parser))(input)?;
    Ok((rest, arbitrary))
}

// https://stackoverflow.com/questions/70630556/parse-allowing-nested-parentheses-in-nom
pub fn take_until_unbalanced(opening_bracket: char, closing_bracket: char) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    let c = it.next().unwrap_or_default();
                    index += c.len_utf8();
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            let error = nom::error::Error::new(i, nom::error::ErrorKind::TakeUntil);
            let error = nom::Err::Error(error);
            Err(error)
        }
    }
}
