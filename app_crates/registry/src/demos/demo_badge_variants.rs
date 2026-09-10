use dioxus::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoBadgeVariants() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4 items-center",
            Badge { "Default" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
        }
    }
}
