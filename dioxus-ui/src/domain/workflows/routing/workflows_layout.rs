use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::workflows::workflows_hero::WorkflowsHero;
use crate::utils::page_transition::{PAGE_OUTLET, retrigger_page_fade};

#[component]
pub fn WorkflowsLayout() -> Element {
    let _route = use_route::<Route>();
    retrigger_page_fade();

    rsx! {
        HeaderDocs {}

        div { "data-name": "__WorkflowsLayout", class: "container flex flex-col gap-20 pb-14",
            WorkflowsHero {}

            div { id: PAGE_OUTLET, class: "flex flex-col gap-20 page__fade",
                Outlet::<Route> {}
            }
        }
    }
}
