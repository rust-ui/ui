use dioxus::prelude::*;
use icons::{GitBranch, RotateCcw};

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon};

#[cfg(target_arch = "wasm32")]
fn alert_reverted() {
    if let Some(window) = web_sys::window() {
        let _ = window.alert_with_message("You clicked the revert button");
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn alert_reverted() {}

#[component]
pub fn DemoMarkerLinkButton() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Marker { href: "#links-and-buttons",
                MarkerIcon { GitBranch {} }
                MarkerContent { "View the pull request" }
            }
            Marker {
                class: "transition-colors hover:text-foreground",
                onclick: move |_| alert_reverted(),
                MarkerIcon { RotateCcw {} }
                MarkerContent { "Revert this change" }
            }
        }
    }
}
