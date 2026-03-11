use leptos::prelude::*;

use crate::leptos::leptos_converter::MdComponentProps;

/* ========================================================== */
/*                      ✨ UTILS ✨                          */
/* ========================================================== */

fn create_anchor_id(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

fn create_heading_view(props: MdComponentProps, tag: &str, class: &str) -> impl IntoView {
    // Use text_content from props if available, otherwise fallback
    let text_content = props.text_content.unwrap_or_else(|| "fallback-heading".to_string());
    let anchor_id = create_anchor_id(&text_content);

    match tag {
        "h2" => view! {
            <h2 id=anchor_id.clone() class=class>
                <a href=format!("#{}", anchor_id) class="no-underline hover:underline">
                    {(props.children)()}
                </a>
            </h2>
        }
        .into_any(),
        "h3" => view! {
            <h3 id=anchor_id.clone() class=class>
                <a href=format!("#{}", anchor_id) class="no-underline hover:underline">
                    {(props.children)()}
                </a>
            </h3>
        }
        .into_any(),
        "h4" => view! {
            <h4 id=anchor_id.clone() class=class>
                <a href=format!("#{}", anchor_id) class="no-underline hover:underline">
                    {(props.children)()}
                </a>
            </h4>
        }
        .into_any(),
        "h5" => view! {
            <h5 id=anchor_id.clone() class=class>
                <a href=format!("#{}", anchor_id) class="no-underline hover:underline">
                    {(props.children)()}
                </a>
            </h5>
        }
        .into_any(),
        "h6" => view! {
            <h6 id=anchor_id.clone() class=class>
                <a href=format!("#{}", anchor_id) class="no-underline hover:underline">
                    {(props.children)()}
                </a>
            </h6>
        }
        .into_any(),
        _ => view! { <div>{(props.children)()}</div> }.into_any(),
    }
}

pub fn md_h2(props: MdComponentProps) -> impl IntoView {
    create_heading_view(props, "h2", "font-heading [&+*]:[code]:text-xl mt-10 scroll-m-28 text-xl font-medium tracking-tight first:mt-0 lg:mt-12 [&+.steps]:mt-0! [&+.steps>h3]:mt-4! [&+h3]:mt-6! [&+p]:mt-4!")
}

pub fn md_h3(props: MdComponentProps) -> impl IntoView {
    create_heading_view(
        props,
        "h3",
        "font-heading mt-12 scroll-m-28 text-lg font-medium tracking-tight [&+p]:mt-4! *:[code]:text-xl",
    )
}

pub fn md_h4(props: MdComponentProps) -> impl IntoView {
    create_heading_view(props, "h4", "font-heading mt-8 scroll-m-28 text-base font-medium tracking-tight")
}

pub fn md_h5(props: MdComponentProps) -> impl IntoView {
    create_heading_view(props, "h5", "mt-8 scroll-m-28 text-base font-medium tracking-tight")
}

pub fn md_h6(props: MdComponentProps) -> impl IntoView {
    create_heading_view(props, "h6", "mt-8 scroll-m-28 text-base font-medium tracking-tight")
}
