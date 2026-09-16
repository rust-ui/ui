use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::hooks::use_pagination::{PaginationContext, use_pagination};

#[component]
pub fn Pagination(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_pagination();
    use_context_provider(|| ctx);

    let merged = tw_merge!("flex justify-center mx-auto w-full", class.as_deref().unwrap_or(""));
    rsx! { nav { class: "{merged}", "aria-label": "pagination", {children} } }
}

#[component]
pub fn PaginationList(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex flex-row gap-1 items-center [&_li:nth-last-child(2):has(a[aria-current=page])~li:last-child]:opacity-0 [&_li:nth-last-child(2):has(a[aria-current=page])~li:last-child]:pointer-events-none",
        class.as_deref().unwrap_or("")
    );
    rsx! { ul { class: "{merged}", {children} } }
}

#[component]
pub fn PaginationItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! { li { class: "{class.as_deref().unwrap_or(\"\")}", {children} } }
}

#[component]
pub fn PaginationLink(page: u32, #[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<PaginationContext>();
    let href = (ctx.page_href)(page);
    let is_active = !(ctx.aria_current)(page).is_empty();

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
        a {
            href: "{href}",
            class: "{merged}",
            "aria-current": (ctx.aria_current)(page),
            onclick: move |e| {
                e.prevent_default();
                (ctx.go_to_page)(page);
            },
            "{page}"
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PageDirection {
    Previous,
    Next,
}

impl PageDirection {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Previous => "previous",
            Self::Next => "next",
        }
    }
}

#[component]
pub fn PaginationNavButton(direction: PageDirection) -> Element {
    let ctx = use_context::<PaginationContext>();

    let (href, is_disabled, target_page) = match direction {
        PageDirection::Previous => ((ctx.prev_href)(), (ctx.is_first_page)(), (ctx.current_page)().saturating_sub(1)),
        PageDirection::Next => ((ctx.next_href)(), false, (ctx.current_page)() + 1),
    };

    let merged = tw_merge!(
        "inline-flex items-center justify-center size-9 rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground cursor-pointer",
        if is_disabled { "opacity-50 pointer-events-none" } else { "" }
    );

    rsx! {
        a {
            href: "{href}",
            class: "{merged}",
            "aria-label": "Go to {direction.as_str()} page",
            onclick: move |e| {
                e.prevent_default();
                if !is_disabled {
                    (ctx.go_to_page)(target_page);
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
                match direction {
                    PageDirection::Previous => rsx! {
                        path { d: "m15 18-6-6 6-6" }
                    },
                    PageDirection::Next => rsx! {
                        path { d: "m9 18 6-6-6-6" }
                    },
                }
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
            span { class: "hidden", "More pages" }
        }
    }
}
