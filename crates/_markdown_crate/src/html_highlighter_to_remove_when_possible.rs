use std::error::Error;

pub const HIGHLIGHT_THEME: &str = "base16-ocean.dark";

#[cfg(feature = "ssr")]
pub fn highlight_html(html: &str) -> Result<String, Box<dyn Error>> {
    ssr::highlight_html(html)
}

#[cfg(not(feature = "ssr"))]
pub fn highlight_html(html: &str) -> Result<String, Box<dyn Error>> {
    // No highlighting on client side - return as is
    Ok(html.to_string())
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[cfg(feature = "ssr")]
mod ssr {
    use std::error::Error;

    use syntect::highlighting::{Color, ThemeSet};
    use syntect::html::highlighted_html_for_string;
    use syntect::parsing::SyntaxSet;

    use super::HIGHLIGHT_THEME;

    pub fn highlight_html(html: &str) -> Result<String, Box<dyn Error>> {
        use html_parser::Dom;

        let dom = Dom::parse(html)?;
        let mut result = String::new();

        for node in &dom.children {
            process_node_for_highlighting(node, &mut result)?;
        }

        Ok(result)
    }

    fn process_node_for_highlighting(node: &html_parser::Node, result: &mut String) -> Result<(), Box<dyn Error>> {
        use html_parser::Node;

        match node {
            Node::Element(element) => {
                if element.name == "pre" && has_code_child_with_language(element) {
                    // This is a code block - process it for highlighting
                    process_code_block_for_highlighting(element, result)?;
                } else {
                    // Regular element - reconstruct with processed children
                    result.push('<');
                    result.push_str(&element.name);

                    // Add attributes
                    if let Some(ref id) = element.id {
                        result.push_str(&format!(" id=\"{}\"", html_escape::encode_double_quoted_attribute(id)));
                    }

                    if !element.classes.is_empty() {
                        let classes = element.classes.join(" ");
                        result
                            .push_str(&format!(" class=\"{}\"", html_escape::encode_double_quoted_attribute(&classes)));
                    }

                    for (attr_name, attr_value) in &element.attributes {
                        if let Some(value) = attr_value {
                            result.push_str(&format!(
                                " {}=\"{}\"",
                                attr_name,
                                html_escape::encode_double_quoted_attribute(value)
                            ));
                        } else {
                            result.push_str(&format!(" {}", attr_name));
                        }
                    }

                    result.push('>');

                    // Process children
                    for child in &element.children {
                        process_node_for_highlighting(child, result)?;
                    }

                    result.push_str(&format!("</{}>", element.name));
                }
            }
            Node::Text(text) => {
                result.push_str(&html_escape::encode_text(text));
            }
            Node::Comment(comment) => {
                result.push_str(&format!("<!--{}-->", comment));
            }
        }

        Ok(())
    }

    fn has_code_child_with_language(pre_element: &html_parser::Element) -> bool {
        use html_parser::Node;

        for child in &pre_element.children {
            if let Node::Element(element) = child {
                if element.name == "code" {
                    // Check if it has a language class
                    return element.classes.iter().any(|class| class.starts_with("language-"));
                }
            }
        }
        false
    }

    fn process_code_block_for_highlighting(
        pre_element: &html_parser::Element,
        result: &mut String,
    ) -> Result<(), Box<dyn Error>> {
        use html_parser::Node;

        // Find the code element and extract language and content
        for child in &pre_element.children {
            if let Node::Element(code_element) = child {
                if code_element.name == "code" {
                    // Extract language from class
                    let language = code_element
                        .classes
                        .iter()
                        .find(|class| class.starts_with("language-"))
                        .map(|class| &class[9..]) // Remove "language-" prefix
                        .filter(|lang| !lang.is_empty());

                    // Extract code content
                    let mut code_content = String::new();
                    extract_text_content(&code_element.children, &mut code_content);

                    // Highlight the code
                    let highlighted = highlight_code_block(&code_content, language, &get_syntax_set(), &get_theme());

                    result.push_str(&highlighted);
                    return Ok(());
                }
            }
        }

        // Fallback: no code element found, just output as regular pre
        result.push_str("<pre>");
        for child in &pre_element.children {
            process_node_for_highlighting(child, result)?;
        }
        result.push_str("</pre>");
        Ok(())
    }

    fn extract_text_content(nodes: &[html_parser::Node], result: &mut String) {
        use html_parser::Node;

        for node in nodes {
            match node {
                Node::Text(text) => {
                    // Decode HTML entities since the text comes from parsed HTML
                    let decoded = html_escape::decode_html_entities(text);
                    result.push_str(&decoded);
                }
                Node::Element(element) => extract_text_content(&element.children, result),
                Node::Comment(_) => {} // Ignore comments in code blocks
            }
        }
    }

    pub fn get_syntax_set() -> SyntaxSet {
        SyntaxSet::load_defaults_newlines()
    }

    pub fn get_theme() -> syntect::highlighting::Theme {
        let ts = ThemeSet::load_defaults();
        ts.themes[HIGHLIGHT_THEME].clone()
    }

    pub fn highlight_code_block(
        code: &str,
        lang: Option<&str>,
        syntax_set: &SyntaxSet,
        theme: &syntect::highlighting::Theme,
    ) -> String {
        if code.trim().is_empty() {
            return format!("<pre><code>{}</code></pre>", html_escape::encode_text(code));
        }

        let syntax = if let Some(lang) = lang {
            syntax_set
                .find_syntax_by_token(lang)
                .or_else(|| syntax_set.find_syntax_by_extension(lang))
                .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
        } else {
            syntax_set.find_syntax_plain_text()
        };

        match highlighted_html_for_string(code, syntax_set, syntax, theme) {
            Ok(highlighted) => {
                let c = theme.settings.background.unwrap_or(Color::WHITE);
                format!(
                    r#"<pre style="background-color:#{:02x}{:02x}{:02x}; padding: 16px; border-radius: 8px; overflow-x: auto; font-size: 13px; font-family: Consolas, 'Liberation Mono', Menlo, Courier, monospace;">{}</pre>"#,
                    c.r, c.g, c.b, highlighted
                )
            }
            Err(_) => {
                format!("<pre><code>{}</code></pre>", html_escape::encode_text(code))
            }
        }
    }

    // TODO. I keep this for the moment. Just in case I need it.
    // pub fn highlight_markdown_events_ssr<'a>(parser: Parser<'a>) -> Vec<Event<'a>> {
    //     let ss = SyntaxSet::load_defaults_newlines();
    //     let ts = ThemeSet::load_defaults();
    //     let theme = &ts.themes[HIGHLIGHT_THEME];

    //     let mut events = Vec::new();
    //     let mut in_code_block = false;
    //     let mut code_block_lang = None;
    //     let mut code_block_content = String::new();

    //     for event in parser {
    //         match event {
    //             Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
    //                 in_code_block = true;
    //                 code_block_lang = Some(lang.to_string());
    //                 code_block_content.clear();
    //             }
    //             Event::End(TagEnd::CodeBlock) => {
    //                 if in_code_block {
    //                     let highlighted_html =
    //                         highlight_code_block(&code_block_content, code_block_lang.as_deref(), &ss, theme);
    //                     events.push(Event::Html(highlighted_html.into()));
    //                     in_code_block = false;
    //                     code_block_lang = None;
    //                 }
    //             }
    //             Event::Text(text) if in_code_block => {
    //                 code_block_content.push_str(&text);
    //             }
    //             _ => {
    //                 if !in_code_block {
    //                     events.push(event);
    //                 }
    //             }
    //         }
    //     }

    //     events
    // }
}
