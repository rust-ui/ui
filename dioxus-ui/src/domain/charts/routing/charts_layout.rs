use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::charts::components::charts_hero::ChartsHero;
use crate::utils::page_transition::{PAGE_OUTLET, retrigger_page_fade};

const APEXCHARTS_JS: Asset = asset!("/public/cdn/apexcharts.5.3.6.min.js");

#[component]
pub fn ChartsLayout() -> Element {
    let _route = use_route::<Route>();
    retrigger_page_fade();

    rsx! {
        document::Script { src: APEXCHARTS_JS }

        HeaderDocs {}

        div { "data-name": "__ChartsLayout", class: "container flex flex-col gap-10",
            ChartsHero {}

            div { id: PAGE_OUTLET, class: "page__fade",
                Outlet::<Route> {}
            }
        }
    }
}
