use dioxus::prelude::*;

use crate::ui::kbd::Kbd;

#[component]
pub fn DemoKbd() -> Element {
    rsx! {
        Kbd { "⌘K" }
    }
}
