use dioxus::prelude::*;

use crate::domain::home::page_home::HomePage;

pub struct HomeRoutes;

impl HomeRoutes {
    pub const LABEL: &'static str = "Home";
}

#[component]
pub fn Home() -> Element {
    rsx! { HomePage {} }
}
