use std::sync::OnceLock;

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::styled_line_to_highlighted_html;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::toml_highlighter::highlight_toml_manually;

const HIGHLIGHT_THEME: &str = "base16-ocean.light";

pub fn highlight_code(code: &str, language: Option<&str>, filename: Option<&str>) -> String {
    #[cfg(feature = "ssr")]
    {
        ssr::highlight_ssr_code(code, language, filename)
    }
    #[cfg(not(feature = "ssr"))]
    {
        wasm::highlight_wasm_code(code, language, filename)
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Core highlighting implementation shared by both SSR and WASM
fn highlight_code_impl(
    code: &str,
    language: Option<&str>,
    filename: Option<&str>,
    syntax_set: &SyntaxSet,
    theme_set: &ThemeSet,
) -> String {
    use crate::highlight_language::HighlightLanguage;

    // Try to get theme - fallback to base16-ocean if not available
    let theme = theme_set
        .themes
        .get(HIGHLIGHT_THEME)
        .or_else(|| theme_set.themes.get("InspiredGitHub"))
        .or_else(|| theme_set.themes.values().next())
        .expect("No themes available");

    // Determine language
    let lang = language.or_else(|| filename.and_then(HighlightLanguage::detect_from_filename)).unwrap_or("plain");

    // Find syntax definition for the language
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

    // Use manual TOML highlighting if Syntect doesn't support TOML
    if lang == "toml" && syntax.name == "Plain Text" {
        return highlight_toml_manually(code);
    }

    // Use syntect's built-in HTML generator with inline styles (no <pre> wrapper)
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut html_output = String::new();

    for line in LinesWithEndings::from(code) {
        let ranges = highlighter.highlight_line(line, syntax_set).unwrap_or_default();
        let line_html = styled_line_to_highlighted_html(&ranges[..], syntect::html::IncludeBackground::No)
            .unwrap_or_else(|_| html_escape::encode_text(line).to_string());
        html_output.push_str(&line_html);
    }

    html_output
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[cfg(feature = "ssr")]
mod ssr {
    use super::*;

    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

    fn get_syntax_set() -> &'static SyntaxSet {
        SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
    }

    fn get_theme_set() -> &'static ThemeSet {
        THEME_SET.get_or_init(ThemeSet::load_defaults)
    }

    pub fn highlight_ssr_code(code: &str, language: Option<&str>, filename: Option<&str>) -> String {
        highlight_code_impl(code, language, filename, get_syntax_set(), get_theme_set())
    }
}

#[cfg(not(feature = "ssr"))]
mod wasm {
    use super::*;

    static WASM_SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    static WASM_THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

    fn get_wasm_syntax_set() -> &'static SyntaxSet {
        WASM_SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
    }

    fn get_wasm_theme_set() -> &'static ThemeSet {
        WASM_THEME_SET.get_or_init(ThemeSet::load_defaults)
    }

    pub fn highlight_wasm_code(code: &str, language: Option<&str>, filename: Option<&str>) -> String {
        highlight_code_impl(code, language, filename, get_wasm_syntax_set(), get_wasm_theme_set())
    }
}
