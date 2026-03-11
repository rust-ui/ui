use std::error::Error;

use html_escape::encode_text;
use pulldown_cmark::{html, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::frontmatter::{extract_content_without_frontmatter, extract_frontmatter};

#[derive(Debug, Clone)]
pub struct MarkdownFileToHtml {
    pub frontmatter: Option<String>,
    pub html_content: String,
}

/* ========================================================== */
/*                      🦀 ENTRYPOINT 🦀                      */
/* ========================================================== */

pub fn convert_md_file_to_html(md_file_content: &str) -> Result<MarkdownFileToHtml, Box<dyn Error>> {
    let frontmatter = extract_frontmatter(md_file_content);
    let md_content = extract_content_without_frontmatter(md_file_content);
    let html_content = md_to_html(md_content);

    Ok(MarkdownFileToHtml { frontmatter, html_content })
}

pub fn convert_md_file_to_html_with_tailwind(md_file_content: &str) -> Result<MarkdownFileToHtml, Box<dyn Error>> {
    let frontmatter = extract_frontmatter(md_file_content);
    let md_content = extract_content_without_frontmatter(md_file_content);
    let html_content = md_to_html_with_tailwind(md_content);

    Ok(MarkdownFileToHtml { frontmatter, html_content })
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

fn md_to_html(md_content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(md_content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn md_to_html_with_tailwind(md_content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(md_content, options);
    let mut html_output = String::new();
    let mut in_list = false;
    let mut list_stack = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let class = match level {
                    HeadingLevel::H1 => "mt-2 text-4xl font-bold tracking-tight scroll-mt-2",
                    HeadingLevel::H2 => "mt-12 text-3xl font-bold tracking-tight scroll-mt-2",
                    HeadingLevel::H3 => "mt-8 text-2xl font-bold tracking-tight scroll-mt-2",
                    HeadingLevel::H4 => "",
                    HeadingLevel::H5 => "",
                    HeadingLevel::H6 => "",
                };
                html_output.push_str(&format!("<h{} class=\"{}\">", level as u8, class));
            }
            Event::End(TagEnd::Heading(level)) => {
                html_output.push_str(&format!("</h{}>", level as u8));
            }
            Event::Start(Tag::Paragraph) => {
                if !in_list {
                    html_output.push_str("<div class=\"my-4\"><p class=\"leading-7\">");
                } else {
                    html_output.push_str("<p class=\"leading-7\">");
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if !in_list {
                    html_output.push_str("</p></div>");
                } else {
                    html_output.push_str("</p>");
                }
            }
            Event::Start(Tag::List(None)) => {
                html_output.push_str("<ul class=\"ml-6 list-disc [&>li]:mt-2\">");
                list_stack.push("ul");
                in_list = true;
            }
            Event::Start(Tag::List(Some(_))) => {
                html_output.push_str("<ol class=\"ml-6 list-decimal [&>li]:mt-2\">");
                list_stack.push("ol");
                in_list = true;
            }
            Event::End(TagEnd::List(_)) => {
                if let Some(tag) = list_stack.pop() {
                    html_output.push_str(&format!("</{}>", tag));
                }
                in_list = !list_stack.is_empty();
            }
            Event::Start(Tag::Item) => {
                html_output.push_str("<li>");
            }
            Event::End(TagEnd::Item) => {
                html_output.push_str("</li>");
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                html_output.push_str(
                    "<pre class=\"overflow-x-auto py-1 mt-6 mb-4 rounded-lg border max-h-[650px] bg-muted\">",
                );

                if lang.is_empty() {
                    html_output.push_str("<code>");
                } else {
                    html_output.push_str(&format!("<code class=\"language-{}\">", lang));
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                html_output.push_str(
                    "<pre class=\"overflow-x-auto py-1 mt-6 mb-4 rounded-lg border max-h-[650px] bg-muted\"><code>",
                );
            }
            Event::End(TagEnd::CodeBlock) => {
                html_output.push_str("</code></pre>");
            }
            Event::Code(text) => {
                html_output.push_str(&format!("<code class=\"relative font-mono rounded bg-muted px-[0.3rem] py-[0.2rem] text-[0.75rem] sm:text-[0.8125rem]\">{}</code>", encode_text(&text)));
            }
            Event::Start(Tag::Link { dest_url, title, .. }) => {
                if title.is_empty() {
                    html_output.push_str(&format!(
                        "<a href=\"{}\" class=\"font-medium no-underline hover:underline underline-offset-4\">",
                        dest_url
                    ));
                } else {
                    html_output.push_str(&format!(
                        "<a href=\"{}\" title=\"{}\" class=\"font-medium no-underline hover:underline underline-offset-4\">",
                        dest_url, title
                    ));
                }
            }
            Event::End(TagEnd::Link) => {
                html_output.push_str("</a>");
            }
            Event::Start(Tag::BlockQuote(_)) => {
                html_output.push_str("<blockquote class=\"pl-6 mt-6 italic border-l-2 [&>*]:text-muted-foreground\">");
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                html_output.push_str("</blockquote>");
            }
            Event::Start(Tag::Table(_)) => {
                html_output.push_str("<table class=\"table-auto border-collapse border border-gray-300 mb-4 w-full\">");
            }
            Event::End(TagEnd::Table) => {
                html_output.push_str("</table>");
            }
            Event::Start(Tag::TableHead) => {
                html_output.push_str("<thead class=\"bg-gray-50\">");
            }
            Event::End(TagEnd::TableHead) => {
                html_output.push_str("</thead>");
            }
            Event::Start(Tag::TableRow) => {
                html_output.push_str("<tr>");
            }
            Event::End(TagEnd::TableRow) => {
                html_output.push_str("</tr>");
            }
            Event::Start(Tag::TableCell) => {
                html_output.push_str("<td class=\"border border-gray-300 px-4 py-2\">");
            }
            Event::End(TagEnd::TableCell) => {
                html_output.push_str("</td>");
            }
            Event::Start(Tag::Emphasis) => {
                html_output.push_str("<em>");
            }
            Event::End(TagEnd::Emphasis) => {
                html_output.push_str("</em>");
            }
            Event::Start(Tag::Strong) => {
                html_output.push_str("<strong>");
            }
            Event::End(TagEnd::Strong) => {
                html_output.push_str("</strong>");
            }
            Event::Start(Tag::Strikethrough) => {
                html_output.push_str("<del class=\"line-through\">");
            }
            Event::End(TagEnd::Strikethrough) => {
                html_output.push_str("</del>");
            }
            Event::Text(text) => {
                html_output.push_str(&encode_text(&text));
            }
            Event::Html(html) => {
                html_output.push_str(&html);
            }
            Event::SoftBreak => {
                html_output.push('\n');
            }
            Event::HardBreak => {
                html_output.push_str("<br>");
            }
            _ => {}
        }
    }

    html_output
}
