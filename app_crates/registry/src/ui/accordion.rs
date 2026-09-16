use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use tw_merge::tw_merge;

static ACCORDION_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[component]
pub fn Accordion(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("divide-y divide-input w-full", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn AccordionItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged =
        tw_merge!("w-full [&:has(>input:checked)>label>svg:last-child]:rotate-180", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn AccordionTrigger(
    #[props(into, optional)] class: Option<String>,
    #[props(default = false)] open: bool,
    children: Element,
) -> Element {
    let id = use_hook(|| {
        let n = ACCORDION_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("accordion_{n}")
    });

    let label_class = tw_merge!(
        "flex justify-between items-center p-3 list-none cursor-pointer [&_svg:not([class*='size-'])]:size-4 peer-focus-visible:ring-2 peer-focus-visible:ring-ring peer-focus-visible:ring-offset-2",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        input {
            id: "{id}",
            r#type: "checkbox",
            class: "overflow-hidden absolute p-0 -m-px w-px h-px whitespace-nowrap border-0 peer",
            style: "clip: rect(0, 0, 0, 0)",
            checked: open,
        }
        label {
            r#for: "{id}",
            class: "{label_class}",
            {children}
            svg {
                class: "transition-all duration-300",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }
}

#[component]
pub fn AccordionContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let inner = tw_merge!("p-3 pt-0", class.as_deref().unwrap_or(""));
    rsx! {
        article {
            class: "grid overflow-hidden transition-all duration-400 grid-rows-[0fr] peer-checked:grid-rows-[1fr]",
            div { "data-name": "__AccordionContentInner", class: "min-h-[0]",
                div { class: "{inner}", {children} }
            }
        }
    }
}

#[component]
pub fn AccordionHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged =
        tw_merge!("flex gap-2 items-center [&_svg:not([class*='size-'])]:size-4", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn AccordionTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-sm font-medium", class.as_deref().unwrap_or(""));
    rsx! { h4 { class: "{merged}", {children} } }
}

#[component]
pub fn AccordionDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-muted-foreground text-sm", class.as_deref().unwrap_or(""));
    rsx! { p { class: "{merged}", {children} } }
}

#[component]
pub fn AccordionLink(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] href: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "grid gap-2.5 items-center p-2 grid-cols-[auto_1fr] [&_svg:not([class*='size-'])]:size-4 hover:bg-muted",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        a { class: "{merged}", href: href.as_deref().unwrap_or("#"), {children} }
    }
}
