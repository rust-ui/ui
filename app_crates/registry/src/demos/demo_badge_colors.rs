use dioxus::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoBadgeColors() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2",
            Badge { variant: BadgeVariant::Success, "Success" }
            Badge { variant: BadgeVariant::Warning, "Warning" }
            Badge { variant: BadgeVariant::Info, "Info" }
        }
    }
}
