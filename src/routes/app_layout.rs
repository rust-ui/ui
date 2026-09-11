use dioxus::prelude::*;

use crate::Route;
use crate::components::app_bottom_nav::AppBottomNav;
use crate::components::command_search_docs::CommandSearchDocsDialog;
use crate::components::navigation::app_wrapper::AppWrapper;
use crate::utils::page_transition::ScrollToTop;

#[component]
pub fn AppLayout() -> Element {
    // Structure mirrors leptos-ui `app.rs`: `bottom__safe` sits on the outer
    // non-scrolling flex column (inside `AppWrapper`), NOT on the `<main>` scroll
    // box, and `AppBottomNav` is a sibling outside that column. Putting the pad on
    // the scroll box instead made the bar overlap content and the scrollbar run
    // under it. The empty `ontouchstart` (on the `AppWrapper` div) lets iOS
    // WKWebView fire CSS `:active` for the nav button press-scale.
    rsx! {
        ScrollToTop {}
        AppWrapper {
            main {
                id: "data-scroll-target",
                class: "overflow-y-auto flex-1 overflow-x-clip",
                Outlet::<Route> {}
            }
        }
        AppBottomNav {}
        // Portal-equivalent: the docs/home headers have `backdrop-filter`,
        // which would trap the dialog's `position: fixed` inside the header.
        // Mounting it here keeps it anchored to the viewport. Triggers in the
        // headers reach it through DOM ids + delegated listeners.
        CommandSearchDocsDialog {}
    }
}
