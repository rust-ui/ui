use dioxus::prelude::*;

use crate::__registry__::demos_sidenav::{SIDENAV_ITEMS, SidenavItem};
use crate::__registry__::sidenav_get_started::SIDENAV_GET_STARTED_ITEMS;
use crate::__registry__::sidenav_hooks::SIDENAV_HOOKS_ITEMS;
use crate::Route;

#[component]
pub fn Sidenav() -> Element {
    let route = use_route::<Route>();
    let is_hooks = matches!(route, Route::DocsHooksIndexPage {} | Route::HookPage { .. });
    let current_slug = match route {
        Route::ComponentPage { ref name } | Route::HookPage { ref name } => Some(name.clone()),
        _ => None,
    };
    let section_title = if is_hooks { "Hooks" } else { "Components" };
    let section_items = if is_hooks { SIDENAV_HOOKS_ITEMS } else { SIDENAV_ITEMS };

    rsx! {
        div { class: "hidden fixed top-14 z-30 md:flex md:sticky md:top-14 md:ml-2 w-[175px] h-[calc(100vh-3.5rem)] shrink-0",
            aside { class: "flex overflow-hidden flex-col flex-1 group/scrollbar-on-hover",
                div { class: "flex overflow-hidden overflow-y-auto overscroll-y-contain flex-col gap-4 pb-4 w-full h-full rounded-[inherit] scrollbar__on_hover",
                    SidenavSection {
                        title: "Get Started",
                        current_slug: current_slug.clone(),
                        items: SIDENAV_GET_STARTED_ITEMS,
                        is_hooks: false,
                    }
                    SidenavSection {
                        title: section_title,
                        current_slug,
                        items: section_items,
                        is_hooks,
                    }
                }
            }
        }
    }
}

#[component]
fn SidenavSection(
    title: &'static str,
    current_slug: Option<String>,
    items: &'static [SidenavItem],
    is_hooks: bool,
) -> Element {
    rsx! {
        div {
            h4 { class: "my-1 text-sm font-semibold", "{title}" }
            ul { class: "ml-1 list-none",
                for item in items {
                    li {
                        if is_hooks {
                            Link {
                                class: "flex items-center gap-1.5",
                                to: Route::HookPage { name: item.slug.to_string() },
                                SidenavLabel { item: *item, is_active: current_slug.as_deref() == Some(item.slug) }
                            }
                        } else {
                            Link {
                                class: "flex items-center gap-1.5",
                                to: Route::ComponentPage { name: item.slug.to_string() },
                                SidenavLabel { item: *item, is_active: current_slug.as_deref() == Some(item.slug) }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidenavLabel(item: SidenavItem, is_active: bool) -> Element {
    let class = if is_active {
        "text-sm text-muted-foreground hover:underline font-bold"
    } else {
        "text-sm text-muted-foreground hover:underline"
    };

    rsx! {
        span { class: "{class}", "{item.label}" }
        if item.is_new {
            span { class: "bg-green-500 rounded-full size-1.5 shrink-0" }
        }
    }
}
