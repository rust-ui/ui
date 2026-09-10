use dioxus::prelude::*;

use crate::ui::status::Status;

#[component]
pub fn DemoStatus() -> Element {
    rsx! {
        Status {
            div { class: "rounded-md size-16 bg-neutral-500" }
        }
    }
}
