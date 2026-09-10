use dioxus::prelude::*;

use crate::hooks::use_media_query::use_media_query;
use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoUseMediaQuery() -> Element {
    let is_md = use_media_query("(min-width: 768px)");
    let is_lg = use_media_query("(min-width: 1024px)");

    rsx! {
        div { class: "flex flex-col gap-4 items-center",
            div { class: "flex gap-2 items-center",
                span { class: "text-sm text-muted-foreground", ">= 768px (md)" }
                if is_md() {
                    Badge { variant: BadgeVariant::Default, "matches" }
                } else {
                    Badge { variant: BadgeVariant::Secondary, "no match" }
                }
            }
            div { class: "flex gap-2 items-center",
                span { class: "text-sm text-muted-foreground", ">= 1024px (lg)" }
                if is_lg() {
                    Badge { variant: BadgeVariant::Default, "matches" }
                } else {
                    Badge { variant: BadgeVariant::Secondary, "no match" }
                }
            }
            p { class: "text-xs text-muted-foreground", "Resize the window to see the signals update." }
        }
    }
}
