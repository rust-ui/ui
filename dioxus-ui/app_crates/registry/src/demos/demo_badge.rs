use dioxus::prelude::*;

use crate::ui::badge::Badge;

#[component]
pub fn DemoBadge() -> Element {
    rsx! {
        Badge { "Default" }
    }
}
