use dioxus::prelude::*;

use crate::Route;
use crate::components::command_search_docs::CommandSearchDocsDialog;
use crate::utils::page_transition::ScrollToTop;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col h-full",
            ScrollToTop {}
            main { id: "data-scroll-target", class: "overflow-y-auto flex-1 overflow-x-clip",
                Outlet::<Route> {}
            }
            // Portal-equivalent: the docs/home headers have `backdrop-filter`,
            // which would trap the dialog's `position: fixed` inside the header.
            // Mounting it here keeps it anchored to the viewport. Triggers in the
            // headers reach it through DOM ids + delegated listeners.
            CommandSearchDocsDialog {}
        }
    }
}
