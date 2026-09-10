use std::collections::HashMap;

use dioxus::prelude::*;
use html_parser::{Dom, Element as HtmlElement, Node};

use crate::components::table_of_contents::{TocItem, create_anchor_id};
use crate::markdown::markdown_to_html;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

pub struct MdNodeProps {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, Option<String>>,
    pub children: Vec<Element>,
    pub text_content: String,
}

/// Function-pointer type for custom components.
/// Using fn pointers (not closures) so the registry can be stored statically.
pub type MdComponent = fn(MdNodeProps) -> Element;

#[derive(Default)]
pub struct MdComponents(HashMap<String, MdComponent>);

impl MdComponents {
    pub fn new() -> Self {
        Self::default()
    }

    /// Keys are stored lowercase so `<DemoCard />` and `"DemoCard"` match transparently.
    pub fn add(&mut self, tag: &'static str, f: MdComponent) -> &mut Self {
        self.0.insert(tag.to_lowercase(), f);
        self
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Extract H2/H3 headings from markdown for the Table of Contents.
pub fn extract_toc_from_md(md: &str) -> Vec<TocItem> {
    let html = markdown_to_html(md);
    let dom = match Dom::parse(&html) {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let mut items = Vec::new();
    for node in &dom.children {
        if let Node::Element(el) = node {
            let depth = match el.name.to_lowercase().as_str() {
                "h2" => 2u8,
                "h3" => 3u8,
                _ => continue,
            };
            let text = extract_text(&el.children);
            if !text.is_empty() {
                let anchor = create_anchor_id(&text);
                items.push(TocItem { title: text, level: depth, anchor });
            }
        }
    }
    items
}

/// Convert a markdown string to a Dioxus Element tree.
pub fn convert_md(md: &str, components: &MdComponents) -> Element {
    let html = markdown_to_html(md);
    let dom = match Dom::parse(&html) {
        Ok(d) => d,
        Err(_) => return rsx! {},
    };
    let children: Vec<Element> = dom.children.iter().map(|n| process_node(n, components)).collect();
    rsx! {
        {children.into_iter()}
    }
}

// ---------------------------------------------------------------------------
// Recursive processing
// ---------------------------------------------------------------------------

fn process_node(node: &Node, components: &MdComponents) -> Element {
    match node {
        Node::Text(t) => {
            let t = t.clone();
            rsx! { "{t}" }
        }
        Node::Element(el) => process_element(el, components),
        Node::Comment(_) => rsx! {},
    }
}

fn extract_text(nodes: &[Node]) -> String {
    nodes
        .iter()
        .map(|n| match n {
            Node::Text(t) => t.clone(),
            Node::Element(el) => extract_text(&el.children),
            _ => String::new(),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn extract_code_block(pre: &HtmlElement) -> Option<(Option<String>, String)> {
    for child in &pre.children {
        if let Node::Element(code_el) = child
            && code_el.name == "code"
        {
            let lang = code_el.classes.iter().find(|c| c.starts_with("language-")).map(|c| c[9..].to_string());
            let text = extract_text(&code_el.children);
            return Some((lang, text));
        }
    }
    None
}

fn process_element(el: &HtmlElement, components: &MdComponents) -> Element {
    let children: Vec<Element> = el.children.iter().map(|n| process_node(n, components)).collect();

    // Check custom registry first.
    if let Some(component) = components.0.get(&el.name.to_lowercase()) {
        let node_props = MdNodeProps {
            id: el.id.clone(),
            classes: el.classes.clone(),
            attributes: el.attributes.clone(),
            children,
            text_content: extract_text(&el.children),
        };
        return component(node_props);
    }

    // Default element rendering with Tailwind classes
    match el.name.to_lowercase().as_str() {
        "h1" => {
            rsx! { h1 { class: "mt-2 text-3xl font-bold tracking-tight scroll-mt-28", {children.into_iter()} } }
        }
        "h2" => {
            let text = extract_text(&el.children);
            let id = create_anchor_id(&text);
            rsx! { h2 { id: "{id}", class: "mt-10 text-xl font-medium tracking-tight lg:mt-12 first:mt-0 scroll-mt-28", {children.into_iter()} } }
        }
        "h3" => {
            let text = extract_text(&el.children);
            let id = create_anchor_id(&text);
            rsx! { h3 { id: "{id}", class: "mt-12 text-lg font-medium tracking-tight scroll-mt-28", {children.into_iter()} } }
        }
        "h4" => {
            rsx! { h4 { class: "mt-8 text-base font-medium tracking-tight scroll-mt-28", {children.into_iter()} } }
        }
        "p" => {
            rsx! { p { class: "leading-relaxed [&:not(:first-child)]:mt-6", {children.into_iter()} } }
        }
        "ul" => rsx! { ul { class: "pl-6 my-6 list-disc", {children.into_iter()} } },
        "ol" => rsx! { ol { class: "pl-6 my-6 list-decimal", {children.into_iter()} } },
        "li" => rsx! { li { class: "mt-2", {children.into_iter()} } },
        "a" => {
            let href = el.attributes.get("href").and_then(|v| v.clone()).unwrap_or_default();
            rsx! { a { class: "font-medium underline underline-offset-4", href: "{href}", {children.into_iter()} } }
        }
        "code" => {
            rsx! { code { class: "relative font-mono break-words rounded-md bg-muted px-[0.3rem] py-[0.2rem] text-[0.8rem]", {children.into_iter()} } }
        }
        "pre" => {
            if let Some((lang, code_text)) = extract_code_block(el) {
                let highlighted = crate::markdown::highlight_code::highlight_code(&code_text, lang.as_deref(), None);
                rsx! {
                    pre {
                        class: "bg-muted rounded-xl overflow-x-auto py-3.5 px-4 mt-6 min-w-0 text-xs",
                        dangerous_inner_html: "{highlighted}",
                    }
                }
            } else {
                rsx! { pre { class: "bg-muted rounded-xl overflow-x-auto py-3.5 px-4 mt-6 min-w-0 text-xs", {children.into_iter()} } }
            }
        }
        "strong" | "b" => rsx! { strong { class: "font-medium", {children.into_iter()} } },
        "em" | "i" => rsx! { em { {children.into_iter()} } },
        "blockquote" => {
            rsx! { blockquote { class: "pl-6 mt-6 italic border-l-2", {children.into_iter()} } }
        }
        "table" => {
            rsx! { div { class: "overflow-y-auto my-6 w-full rounded-xl border", table { class: "w-full text-sm border-none", {children.into_iter()} } } }
        }
        "thead" => rsx! { thead { {children.into_iter()} } },
        "tbody" => rsx! { tbody { {children.into_iter()} } },
        "tr" => rsx! { tr { class: "m-0 border-b", {children.into_iter()} } },
        "th" => rsx! { th { class: "py-2 px-4 font-bold text-left", {children.into_iter()} } },
        "td" => {
            rsx! { td { class: "py-2 px-4 text-left whitespace-nowrap", {children.into_iter()} } }
        }
        "hr" => rsx! { hr { class: "my-4 md:my-8" } },
        _ => rsx! { {children.into_iter()} },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_stores_key_lowercase() {
        let mut c = MdComponents::new();
        c.add("DemoCard", |_| rsx! {});
        assert!(c.0.contains_key("democard"), "key should be stored as lowercase");
        assert!(!c.0.contains_key("DemoCard"), "original case should not be stored");
    }

    #[test]
    fn add_pascal_matches_html_parser_lowercase() {
        // html_parser lowercases all tag names, so <DemoCard /> → "democard"
        // registering "DemoCard" should produce a hit for "democard"
        let mut c = MdComponents::new();
        c.add("DemoCard", |_| rsx! {});
        assert!(c.0.get("democard").is_some());
    }

    #[test]
    fn add_kebab_also_works() {
        let mut c = MdComponents::new();
        c.add("demo-card", |_| rsx! {});
        assert!(c.0.contains_key("demo-card"));
    }

    #[test]
    fn parse_md_extracts_frontmatter() {
        use crate::markdown::parse_md;
        let raw = "+++\ntitle = \"Button\"\ndescription = \"A button.\"\n+++\n## Usage\nHello";
        let (fm, body) = parse_md(raw);
        assert_eq!(fm.title, "Button");
        assert_eq!(fm.description, "A button.");
        assert!(body.contains("## Usage"));
    }

    #[test]
    fn parse_md_no_frontmatter_returns_full_body() {
        use crate::markdown::parse_md;
        let raw = "## Just body";
        let (fm, body) = parse_md(raw);
        assert_eq!(fm.title, "");
        assert_eq!(body, raw);
    }

    #[test]
    fn markdown_to_html_renders_heading() {
        use crate::markdown::markdown_to_html;
        let html = markdown_to_html("## Hello");
        assert!(html.contains("<h2>") || html.contains("<h2 "));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn pascal_case_tag_passes_through_pulldown() {
        use crate::markdown::markdown_to_html;
        let html = markdown_to_html("\n<DemoButton />\n");
        eprintln!("pulldown output: {:?}", html);
        assert!(!html.contains("&lt;"), "pulldown-cmark escaped the PascalCase tag");
    }

    #[test]
    fn html_parser_parses_pascal_case_element() {
        let html = "<DemoButton />";
        let dom = Dom::parse(html).unwrap();
        eprintln!("dom children: {:?}", dom.children);
        // html_parser preserves original case — name is "DemoButton" not "demobutton"
        let has_element =
            dom.children.iter().any(|n| matches!(n, Node::Element(e) if e.name.to_lowercase() == "demobutton"));
        assert!(has_element, "html_parser did not parse <DemoButton /> as element");
    }

    #[test]
    fn markdown_to_html_renders_table() {
        use crate::markdown::markdown_to_html;
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = markdown_to_html(md);
        assert!(html.contains("<table"));
        assert!(html.contains("<td>"));
    }
}
