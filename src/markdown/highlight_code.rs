use std::sync::OnceLock;

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{IncludeBackground, styled_line_to_highlighted_html};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::markdown::highlight_language::HighlightLanguage;
use crate::markdown::toml_highlighter::highlight_toml_manually;

const HIGHLIGHT_THEME: &str = "base16-ocean.light";

/// Returns highlighted HTML spans (inline styles, no `<pre>` wrapper).
/// Runs syntect on both server (SSR) and client (WASM), matching the leptos site.
pub fn highlight_code(code: &str, language: Option<&str>, filename: Option<&str>) -> String {
    #[cfg(feature = "server")]
    {
        ssr::highlight(code, language, filename)
    }
    #[cfg(not(feature = "server"))]
    {
        wasm::highlight(code, language, filename)
    }
}

/// Core highlighting implementation shared by both SSR and WASM.
fn highlight_impl(
    code: &str,
    language: Option<&str>,
    filename: Option<&str>,
    syntax_set: &SyntaxSet,
    theme_set: &ThemeSet,
) -> String {
    let theme = theme_set
        .themes
        .get(HIGHLIGHT_THEME)
        .or_else(|| theme_set.themes.get("InspiredGitHub"))
        .or_else(|| theme_set.themes.values().next())
        .expect("no syntect theme available");

    let lang = language.or_else(|| filename.and_then(HighlightLanguage::detect_from_filename)).unwrap_or("plain");

    let syntax = match lang {
        "rust" => syntax_set.find_syntax_by_name("Rust"),
        "bash" => syntax_set
            .find_syntax_by_name("Bourne Again Shell (bash)")
            .or_else(|| syntax_set.find_syntax_by_name("Shell-Unix-Generic"))
            .or_else(|| syntax_set.find_syntax_by_extension("sh")),
        "toml" => syntax_set.find_syntax_by_name("TOML"),
        _ => syntax_set.find_syntax_by_extension(lang),
    }
    .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    if lang == "toml" && syntax.name == "Plain Text" {
        return highlight_toml_manually(code);
    }

    let mut hl = HighlightLines::new(syntax, theme);
    let mut out = String::new();

    for line in LinesWithEndings::from(code) {
        let ranges = hl.highlight_line(line, syntax_set).unwrap_or_default();
        let line_html = styled_line_to_highlighted_html(&ranges, IncludeBackground::No)
            .unwrap_or_else(|_| html_escape::encode_text(line).to_string());
        out.push_str(&line_html);
    }

    out
}

#[cfg(feature = "server")]
mod ssr {
    use super::*;

    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

    fn syntax_set() -> &'static SyntaxSet {
        SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
    }

    fn theme_set() -> &'static ThemeSet {
        THEME_SET.get_or_init(ThemeSet::load_defaults)
    }

    pub fn highlight(code: &str, language: Option<&str>, filename: Option<&str>) -> String {
        highlight_impl(code, language, filename, syntax_set(), theme_set())
    }
}

#[cfg(not(feature = "server"))]
mod wasm {
    use super::*;

    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

    fn syntax_set() -> &'static SyntaxSet {
        SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
    }

    fn theme_set() -> &'static ThemeSet {
        THEME_SET.get_or_init(ThemeSet::load_defaults)
    }

    pub fn highlight(code: &str, language: Option<&str>, filename: Option<&str>) -> String {
        highlight_impl(code, language, filename, syntax_set(), theme_set())
    }
}
