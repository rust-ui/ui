use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::app_header::AppHeader;

#[component]
pub fn TestLayout() -> Element {
    rsx! {
        AppHeader {}
        div { class: "container flex flex-col gap-10 py-10",
            Outlet::<Route> {}
        }
    }
}
