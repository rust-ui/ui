use std::collections::HashMap;

use html_escape;
use html_parser::{Dom, Element};
use leptos::logging::warn;
use leptos::prelude::*;

use crate::convert_md_file_to_html;

#[component]
/// Converts md_content into Leptos HTML elements.
/// Custom components can be used in the md_content.
pub fn LeptosConverter(md_content: String, md_components: MdComponents) -> impl IntoView {
    let parsed = convert_md_file_to_html(&md_content).expect("invalid md");

    let dom = Dom::parse(&parsed.html_content).expect("invalid html");

    let mut root_views = vec![];
    for node in dom.children {
        if let Some(el) = node.element() {
            root_views.push(process_element(el, &md_components));
        }
    }

    view! { <div data-name="__LeptosConverter">{root_views}</div> }
}

pub struct MdComponentProps {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, Option<String>>,
    pub children: Children,
    pub text_content: Option<String>,
}

#[derive(Default)]
pub struct MdComponents {
    components: HashMap<String, Box<dyn Fn(MdComponentProps) -> View<AnyView>>>,
}

impl MdComponents {
    pub fn new() -> Self {
        Self { components: HashMap::new() }
    }

    pub fn extend(&mut self, other: MdComponents) {
        self.components.extend(other.components);
    }

    /// Register a component with no props.
    pub fn add<F, IV>(&mut self, name: &str, component: F)
    where
        F: Fn() -> IV + 'static,
        IV: IntoView + 'static,
    {
        self.components.insert(
            name.to_string(),
            Box::new(move |_| {
                let view = component().into_view();
                View::new(view.into_any())
            }),
        );
    }

    /// Register a component with props handling.
    pub fn add_with_props<F, IV, Props, PropsFn>(&mut self, name: &str, component: F, props_adapter: PropsFn)
    where
        F: Fn(Props) -> IV + 'static,
        IV: IntoView + 'static,
        PropsFn: Fn(MdComponentProps) -> Props + 'static,
    {
        self.components.insert(
            name.to_string(),
            Box::new(move |props| {
                let props = props_adapter(props);
                let view = component(props).into_view();
                View::new(view.into_any())
            }),
        );
    }

    /// Register a component with anchor link functionality that takes MdComponentProps directly.
    /// This method provides access to text_content for generating anchor IDs.
    pub fn add_component_with_anchor_support<F, IV>(&mut self, name: &str, component: F)
    where
        F: Fn(MdComponentProps) -> IV + 'static,
        IV: IntoView + 'static,
    {
        self.components.insert(
            name.to_string(),
            Box::new(move |props| {
                let view = component(props).into_view();
                View::new(view.into_any())
            }),
        );
    }

    fn get(&self, name: &str) -> Option<&dyn Fn(MdComponentProps) -> View<AnyView>> {
        self.components.get(name).map(|v| &**v)
    }
}

/// Extracts plain text content from HTML nodes recursively
fn extract_text_from_nodes(nodes: &[html_parser::Node]) -> String {
    use html_parser::Node;
    let mut result = String::new();

    for node in nodes {
        match node {
            Node::Text(text) => {
                // Decode HTML entities since the text comes from parsed HTML
                let decoded = html_escape::decode_html_entities(text);
                result.push_str(&decoded);
            }
            Node::Element(element) => {
                let child_text = extract_text_from_nodes(&element.children);
                result.push_str(&child_text);
            }
            Node::Comment(_) => {} // Ignore comments
        }
    }

    result.trim().to_string()
}

/// Recursively processes child nodes from HTML parser into Leptos Views.
/// Converts Element nodes to Views via process_element and Text nodes to text Views.
fn process_children_recursively_to_views(
    children: &[html_parser::Node],
    components: &MdComponents,
) -> Vec<View<AnyView>> {
    let mut child_views = vec![];
    for child in children {
        match child {
            html_parser::Node::Element(child_el) => {
                child_views.push(process_element(child_el, components));
            }
            html_parser::Node::Text(text) => {
                let decoded_text = html_escape::decode_html_entities(text);
                child_views.push(View::new(view! { {decoded_text} }.into_any()));
            }
            _ => {}
        }
    }
    child_views
}

pub fn process_element(el: &Element, components: &MdComponents) -> View<AnyView> {
    if let Some(component) = components.get(&el.name) {
        // Process children as Views directly instead of HTML strings
        let child_views = process_children_recursively_to_views(&el.children, components);

        // Create children closure from the processed views
        let children = Box::new(move || child_views.into_any()) as Children;

        // Extract text content for heading anchor generation
        let text_content = if el.name.starts_with('h') && el.name.len() == 2 {
            // For headings (h1, h2, h3, etc.), extract text content
            Some(extract_text_from_nodes(&el.children))
        } else {
            None
        };

        let cmp = component(MdComponentProps {
            id: el.id.clone(),
            classes: el.classes.clone(),
            attributes: el.attributes.clone(),
            children,
            text_content,
        });
        return cmp;
    }

    let children = process_children_recursively_to_views(&el.children, components);

    let element_name_string = el.name.clone();

    match element_name_string.as_str() {
        "h1" => View::new(view! { <h1 class="mt-2 text-3xl font-bold tracking-tight font-heading scroll-m-28">{children}</h1> }.into_any()),
        "h2" => View::new(view! {
            <h2 class="mt-10 text-xl font-medium tracking-tight lg:mt-12 first:mt-0 font-heading [&+*]:[code]:text-xl scroll-m-28 [&+.steps]:mt-0! [&+.steps>h3]:mt-4! [&+h3]:mt-6! [&+p]:mt-4!">
                {children}
            </h2>
        }.into_any()),
        "h3" => View::new(view! {
            <h3 class="mt-12 text-lg font-medium tracking-tight font-heading scroll-m-28 [&+p]:mt-4! *:[code]:text-xl">
                {children}
            </h3>
        }.into_any()),
        "h4" => View::new(view! { <h4 class="mt-8 text-base font-medium tracking-tight font-heading scroll-m-28">{children}</h4> }.into_any()),
        "h5" => View::new(view! { <h5 class="mt-8 text-base font-medium tracking-tight scroll-m-28">{children}</h5> }.into_any()),
        "h6" => View::new(view! { <h6 class="mt-8 text-base font-medium tracking-tight scroll-m-28">{children}</h6> }.into_any()),
        "p" => View::new(view! { <p class="leading-relaxed [&:not(:first-child)]:mt-6">{children}</p> }.into_any()),
        "div" => View::new(view! { <div>{children}</div> }.into_any()),
        "span" => View::new(view! { <span>{children}</span> }.into_any()),
        "a" => {
            let href = el.attributes.get("href").and_then(|v| v.clone()).unwrap_or_default();
            View::new(view! {
                <a href=href class="font-medium underline underline-offset-4">
                    {children}
                </a>
            }.into_any())
        }
        "ul" => View::new(view! { <ul class="my-6 pl-6 list-disc">{children}</ul> }.into_any()),
        "ol" => View::new(view! { <ol class="my-6 pl-6 list-decimal">{children}</ol> }.into_any()),
        "li" => View::new(view! { <li class="mt-2">{children}</li> }.into_any()),
        "pre" => {
            // Extract language from child code elements
            let language = el.children.iter()
                .filter_map(|child| child.element())
                .find(|child_el| child_el.name == "code")
                .and_then(|code_el| {
                    code_el.classes.iter()
                        .find(|class| class.starts_with("language-"))
                        .map(|class| &class[9..]) // Remove "language-" prefix
                        .filter(|lang| !lang.is_empty())
                })
                .unwrap_or("");

            View::new(view! {
                <pre
                    data-name="__LeptosConverter__Highlighter"
                    data-language=language
                    class="overflow-x-auto overflow-y-auto overscroll-y-auto overscroll-x-contain py-3.5 px-4 min-w-0 text-xs outline-none no-scrollbar"
                >
                    {children}
                </pre>
            }.into_any())
        },
        "code" => {
            use crate::highlight_code::highlight_code;

            let code_content = extract_text_from_nodes(&el.children);

            // Extract language from classes (look for language-* pattern)
            let language = el.classes.iter()
                .find(|class| class.starts_with("language-"))
                .map(|class| &class[9..]) // Remove "language-" prefix
                .filter(|lang| !lang.is_empty());

            // Inline code (no language class) gets pill styling; block code gets highlighted
            if language.is_none() {
                View::new(view! {
                    <code class="relative font-mono break-words rounded-md outline-none bg-muted px-[0.3rem] py-[0.2rem] text-[0.8rem]">
                        {code_content}
                    </code>
                }.into_any())
            } else {
                View::new(view! { <code inner_html=highlight_code(&code_content, language, None) /> }.into_any())
            }
        },
        "blockquote" => View::new(view! { <blockquote class="pl-6 mt-6 italic border-l-2">{children}</blockquote> }.into_any()),
        "img" => {
            let src = el.attributes.get("src").and_then(|v| v.clone()).unwrap_or_default();
            let alt = el.attributes.get("alt").and_then(|v| v.clone()).unwrap_or_default();
            View::new(view! { <img src=src alt=alt class="rounded-md" /> }.into_any())
        }
        "strong" => View::new(view! { <strong class="font-medium">{children}</strong> }.into_any()),
        "em" => View::new(view! { <em>{children}</em> }.into_any()),
        "br" => View::new(view! { <br /> }.into_any()),
        "hr" => View::new(view! { <hr class="my-4 md:my-8" /> }.into_any()),
        "figure" => View::new(view! { <figure>{children}</figure> }.into_any()),
        "figcaption" => View::new(view! {
            <figcaption class="flex gap-2 items-center text-muted-foreground [&_svg]:size-4 [&_svg]:opacity-70">
                {children}
            </figcaption>
        }.into_any()),
        // Table elements
        "table" => View::new(view! {
            <div class="overflow-y-auto my-6 w-full rounded-xl border no-scrollbar">
                <table class="overflow-hidden relative w-full text-sm border-none [&_tbody_tr:last-child]:border-b-0">
                    {children}
                </table>
            </div>
        }.into_any()),
        "thead" => View::new(view! { <thead>{children}</thead> }.into_any()),
        "tbody" => View::new(view! { <tbody>{children}</tbody> }.into_any()),
        "tr" => View::new(view! { <tr class="m-0 border-b">{children}</tr> }.into_any()),
        "th" => View::new(view! {
            <th class="py-2 px-4 font-bold text-left [&[align=center]]:text-center [&[align=right]]:text-right">
                {children}
            </th>
        }.into_any()),
        "td" => View::new(view! {
            <td class="py-2 px-4 text-left whitespace-nowrap [&[align=center]]:text-center [&[align=right]]:text-right">
                {children}
            </td>
        }.into_any()),
        _ => {
            let unknown_element = element_name_string.clone();
            warn!("[LEPTOS_CONVERTER]: Unknown element {unknown_element}");
            View::new(view! { <div>{"[LEPTOS_CONVERTER]: Unknown element: "}{unknown_element}</div> }.into_any())
        }
    }
}
