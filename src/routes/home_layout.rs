use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::app_header::AppHeader;

#[component]
pub fn HomeLayout() -> Element {
    rsx! {
        AppHeader {}
        Outlet::<Route> {}
    }
}
