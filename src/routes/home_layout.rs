use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_home::HeaderHome;

#[component]
pub fn HomeLayout() -> Element {
    rsx! {
        HeaderHome {}
        Outlet::<Route> {}
    }
}
