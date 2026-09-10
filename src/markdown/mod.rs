pub mod converter;
pub mod highlight_code;
pub mod highlight_language;
pub mod toml_highlighter;

use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Frontmatter {
    pub title: String,
    #[serde(default)]
    pub description: String,
}

/// Parse `+++\ntoml\n+++\nbody` into (Frontmatter, body_markdown).
pub fn parse_md(raw: &str) -> (Frontmatter, &str) {
    let content = raw.trim();
    if let Some(rest) = content.strip_prefix("+++\n")
        && let Some(end) = rest.find("\n+++")
    {
        let toml_str = &rest[..end];
        let body = &rest[end + 4..]; // skip "\n+++"
        if let Ok(fm) = toml::from_str::<Frontmatter>(toml_str) {
            return (fm, body.trim_start_matches('\n'));
        }
    }
    (Frontmatter { title: String::new(), description: String::new() }, raw)
}

pub fn markdown_to_html(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, opts);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
