use dioxus::prelude::*;

use crate::Route;
use crate::components::app_bottom_nav::AppBottomNav;
use crate::components::command_search_docs::CommandSearchDocsDialog;
use crate::utils::page_transition::ScrollToTop;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        // Empty `ontouchstart` lets iOS WKWebView fire CSS `:active` (used by the
        // bottom-nav button press-scale). WebKit-level quirk, not Tauri-specific.
        div { class: "flex flex-col h-full", ontouchstart: move |_| {},
            ScrollToTop {}
            main {
                id: "data-scroll-target",
                // `bottom__safe`: pb = --bottom__nav__height below `sm`, 0 at sm+,
                // so the last content row scrolls clear of the fixed bar.
                class: "overflow-y-auto flex-1 overflow-x-clip bottom__safe",
                Outlet::<Route> {}
            }
            AppBottomNav {}
            // Portal-equivalent: the docs/home headers have `backdrop-filter`,
            // which would trap the dialog's `position: fixed` inside the header.
            // Mounting it here keeps it anchored to the viewport. Triggers in the
            // headers reach it through DOM ids + delegated listeners.
            CommandSearchDocsDialog {}
        }
    }
}
