mod parser;
#[cfg(test)]
mod parser_test;

pub(crate) use parser::{parse_tailwind, take_until_unbalanced};

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct AstStyle<'a> {
    pub source: &'a str,
    /// Is a `!important` style
    pub important: bool,
    /// Is a negative style
    pub negative: bool,
    /// `hover:`, `focus:`, etc.
    pub variants: Vec<&'a str>,
    /// parts of style separated by `-`
    pub elements: Vec<&'a str>,
    /// Is a arbitrary value
    pub arbitrary: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Default)]
struct AstElements<'a> {
    /// `name-space`
    pub elements: Vec<&'a str>,
}

#[derive(Clone, Debug, PartialEq)]
enum ASTVariant<'a> {
    // hover, focus, aria-checked
    Normal(&'a str),
    // data-[size=large]
    // supports-[display: grid]
    DataAttribute(&'a str),
    // [&:nth-child(3)]
    ArbitraryAttribute(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AstParseOptions<'a> {
    /// Custom prefix for modifiers in Tailwind classes
    /// <https://tailwindcss.com/docs/configuration#prefix>
    pub prefix: &'a str,
    /// Custom separator for modifiers in Tailwind classes
    /// <https://tailwindcss.com/docs/configuration#separator>
    pub separator: &'a str,
}

impl Default for AstParseOptions<'static> {
    fn default() -> Self {
        Self { prefix: "", separator: ":" }
    }
}
