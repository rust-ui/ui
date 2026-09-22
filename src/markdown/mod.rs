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
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub publish_date: String,
    #[serde(default)]
    pub last_updated: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub author_role: String,
    #[serde(default)]
    pub author_image: String,
    #[serde(default)]
    pub short_title: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub image: String,
}

/// Parse `+++\ntoml\n+++\nbody` into (`Frontmatter`, `body_markdown`).
#[must_use]
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
    (
        Frontmatter {
            title: String::new(),
            description: String::new(),
            category: String::new(),
            publish_date: String::new(),
            last_updated: String::new(),
            author: String::new(),
            author_role: String::new(),
            author_image: String::new(),
            short_title: String::new(),
            keywords: Vec::new(),
            image: String::new(),
        },
        raw,
    )
}

#[must_use]
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
