use dioxus::prelude::*;

use crate::components::hooks::use_theme_mode::use_theme_provider;
use crate::components::layout::app_bottom_nav::AppBottomNav;
use crate::components::layout::app_wrapper::AppWrapper;
use crate::components::layout::header::Header;
use crate::domain::home::routes::Home;
use crate::domain::item::routing::{ItemDetails, ItemList};
use crate::domain::settings::routes::Settings;

const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/settings")]
        Settings {},
        #[nest("/items")]
            #[route("/")]
            ItemList {},
            #[route("/:id")]
            ItemDetails { id: String },
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    use_theme_provider();

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn AppLayout() -> Element {
    rsx! {
        AppWrapper {
            Header {}
            main { class: "overflow-y-auto flex-1 overflow-x-clip h-full",
                Outlet::<Route> {}
            }
            AppBottomNav {}
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-full gap-4 p-8",
            p { class: "text-2xl font-semibold", "404" }
            p { class: "text-muted-foreground", "Page not found: /{route.join(\"/\")}" }
        }
    }
}
