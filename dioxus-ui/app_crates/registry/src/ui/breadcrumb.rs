use dioxus::prelude::*;
use icons::{ChevronRight, Ellipsis};
use tw_merge::tw_merge;

#[component]
pub fn Breadcrumb(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        nav { "aria-label": "breadcrumb", class: "{class.as_deref().unwrap_or(\"\")}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbList(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex flex-wrap gap-1 items-center text-sm break-words sm:gap-2 text-muted-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! { ol { class: "{merged}", {children} } }
}

#[component]
pub fn BreadcrumbItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "inline-flex gap-1 items-center [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );
    rsx! { li { class: "{merged}", {children} } }
}

#[component]
pub fn BreadcrumbLink(
    #[props(into)] href: String,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("transition-colors hover:text-foreground", class.as_deref().unwrap_or(""));
    rsx! { a { class: "{merged}", href: "{href}", {children} } }
}

#[component]
pub fn BreadcrumbPage(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("font-normal text-foreground", class.as_deref().unwrap_or(""));
    rsx! {
        span {
            class: "{merged}",
            role: "link",
            "aria-disabled": "true",
            "aria-current": "page",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbEllipsis(#[props(into, optional)] class: Option<String>) -> Element {
    rsx! {
        button {
            "data-name": "RootEllipsisBtn",
            class: "flex items-center gap-1",
            "aria-haspopup": "menu",
            "aria-expanded": "false",
            "data-state": "closed",
            span {
                "data-name": "RootEllipsis",
                class: "flex items-center justify-center size-4",
                role: "presentation",
                "aria-hidden": "true",
                Ellipsis { class: class.as_deref().unwrap_or("") }
                span { class: "hidden", "More" }
            }
            span { class: "hidden", "Toggle menu" }
        }
    }
}

#[component]
pub fn BreadcrumbSeparator(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!("[&>svg]:size-3.5 [&_svg:not([class*='size-'])]:size-4", class.as_deref().unwrap_or(""));
    rsx! {
        li { class: "{merged}", role: "presentation", "aria-hidden": "true",
            ChevronRight {}
        }
    }
}
