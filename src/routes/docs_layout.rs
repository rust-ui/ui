use dioxus::prelude::*;
use dioxus::router::use_route;

use crate::__registry__::static_md_registry::{find_docs_component_entry, find_hook_entry};
use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;
use crate::components::sidenav::Sidenav;
use crate::components::table_of_contents::{TableOfContents, TocItem};
use crate::markdown::converter::extract_toc_from_md;
use crate::utils::page_transition::{PAGE_OUTLET, retrigger_page_fade};

#[component]
pub fn DocsLayout() -> Element {
    // Derive the TOC from the current route so it always tracks the page being
    // shown. `use_route` is reactive, so this recomputes on client-side nav
    // without an effect or cross-component signal write.
    let route = use_route::<Route>();
    retrigger_page_fade();
    let toc_items: Vec<TocItem> = match &route {
        Route::ComponentPage { name } => {
            find_docs_component_entry(name).map(|e| extract_toc_from_md(e.body_md())).unwrap_or_default()
        }
        Route::HookPage { name } => find_hook_entry(name).map(|e| extract_toc_from_md(e.body_md())).unwrap_or_default(),
        _ => Vec::new(),
    };

    rsx! {
        HeaderDocs {}
        div { class: "flex-1",
            div { class: "container mx-auto flex items-start",
                Sidenav {}
                div { id: PAGE_OUTLET, class: "flex-1 min-w-0 page__fade",
                    Outlet::<Route> {}
                }
                TableOfContents { toc_items }
            }
        }
    }
}
