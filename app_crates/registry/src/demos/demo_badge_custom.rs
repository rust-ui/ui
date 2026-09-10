use dioxus::prelude::*;

use crate::ui::badge::Badge;

#[component]
pub fn DemoBadgeCustom() -> Element {
    rsx! {
        div { class: "flex gap-2 items-center",
            Badge { class: "bg-sky-500", "Custom" }
        }
    }
}
