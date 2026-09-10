use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::blocks::components::blocks_hero::BlocksHero;
use crate::utils::page_transition::{PAGE_OUTLET, retrigger_page_fade};

#[component]
pub fn BlocksLayout() -> Element {
    // Subscribe to the router so this re-runs on nested route changes, then
    // replay the intro fade on the outlet.
    let _route = use_route::<Route>();
    retrigger_page_fade();

    rsx! {
        HeaderDocs {}

        div { "data-name": "__BlockLayout", class: "container flex flex-col gap-20 pb-14",
            BlocksHero {}

            div { id: PAGE_OUTLET, class: "flex flex-col gap-20 page__fade",
                Outlet::<Route> {}
            }
        }
    }
}
