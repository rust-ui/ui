use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Pagination(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex justify-center mx-auto w-full", class.as_deref().unwrap_or(""));
    rsx! { nav { class: "{merged}", "aria-label": "pagination", {children} } }
}

#[component]
pub fn PaginationList(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex flex-row gap-1 items-center", class.as_deref().unwrap_or(""));
    rsx! { ul { class: "{merged}", {children} } }
}

#[component]
pub fn PaginationItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { li { class: "{class.as_deref().unwrap_or(\"\")}", {children} } }
}

#[component]
pub fn PaginationLink(
    #[props(into, optional)] class: Option<String>,
    page: u32,
    #[props(default = false)] is_active: bool,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center justify-center size-9 rounded-md text-sm font-medium transition-colors cursor-pointer",
        if is_active {
            "bg-primary text-primary-foreground hover:bg-primary/90"
        } else {
            "hover:bg-accent hover:text-accent-foreground text-muted-foreground"
        },
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            class: "{merged}",
            "aria-current": if is_active { "page" } else { "" },
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            "{page}"
        }
    }
}

#[component]
pub fn PaginationPrev(
    #[props(default = false)] disabled: bool,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center justify-center size-9 rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground cursor-pointer",
        if disabled { "opacity-50 pointer-events-none" } else { "" }
    );
    rsx! {
        button {
            class: "{merged}",
            disabled: disabled,
            "aria-label": "Go to previous page",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-4",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m15 18-6-6 6-6" }
            }
        }
    }
}

#[component]
pub fn PaginationNext(
    #[props(default = false)] disabled: bool,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center justify-center size-9 rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground cursor-pointer",
        if disabled { "opacity-50 pointer-events-none" } else { "" }
    );
    rsx! {
        button {
            class: "{merged}",
            disabled: disabled,
            "aria-label": "Go to next page",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-4",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}

#[component]
pub fn PaginationEllipsis(#[props(into, optional)] class: Option<String>) -> Element {
    let merged =
        tw_merge!("flex items-center justify-center size-9 text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! {
        span { class: "{merged}", "aria-hidden": "true",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-4",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "M5 12h.01M12 12h.01M19 12h.01" }
            }
        }
    }
}
