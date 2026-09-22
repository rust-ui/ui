use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::app_header::AppHeader;
use crate::domain::workflows::workflows_hero::WorkflowsHero;
use crate::utils::page_transition::{PAGE_OUTLET, retrigger_page_fade};

#[component]
pub fn WorkflowsLayout() -> Element {
    let _route = use_route::<Route>();
    retrigger_page_fade();

    rsx! {
        AppHeader {}

        div { "data-name": "__WorkflowsLayout", class: "container flex flex-col gap-20 pb-14",
            WorkflowsHero {}

            div { id: PAGE_OUTLET, class: "flex flex-col gap-20 page__fade",
                Outlet::<Route> {}
            }
        }
    }
}
