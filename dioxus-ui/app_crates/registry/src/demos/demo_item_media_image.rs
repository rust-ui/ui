use dioxus::prelude::*;
use icons::ChevronRight;

use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemMedia, ItemMediaVariant, ItemSeparator,
    ItemTitle,
};

#[component]
pub fn DemoItemMediaImage() -> Element {
    rsx! {
        ItemGroup { class: "w-full max-w-md rounded-md border",
            Item { class: "py-3 px-4 rounded-none",
                ItemMedia { variant: ItemMediaVariant::Image,
                    div { class: "bg-gradient-to-br from-blue-400 to-blue-600 size-full" }
                }
                ItemContent {
                    ItemTitle { "Getting Started with Dioxus" }
                    ItemDescription { "Learn how to build reactive web apps in Rust." }
                }
                ItemActions {
                    ChevronRight { class: "size-4 text-muted-foreground" }
                }
                ItemFooter {
                    Badge { variant: BadgeVariant::Secondary, "Tutorial" }
                    span { class: "text-xs text-muted-foreground", "Jan 2026" }
                }
            }
            ItemSeparator {}
            Item { class: "py-3 px-4 rounded-none",
                ItemMedia { variant: ItemMediaVariant::Image,
                    div { class: "bg-gradient-to-br from-violet-400 to-violet-600 size-full" }
                }
                ItemContent {
                    ItemTitle { "Tailwind CSS Best Practices" }
                    ItemDescription { "Utility-first CSS patterns for modern apps." }
                }
                ItemActions {
                    ChevronRight { class: "size-4 text-muted-foreground" }
                }
                ItemFooter {
                    Badge { variant: BadgeVariant::Secondary, "CSS" }
                    span { class: "text-xs text-muted-foreground", "Feb 2026" }
                }
            }
            ItemSeparator {}
            Item { class: "py-3 px-4 rounded-none",
                ItemMedia { variant: ItemMediaVariant::Image,
                    div { class: "bg-gradient-to-br from-emerald-400 to-emerald-600 size-full" }
                }
                ItemContent {
                    ItemTitle { "Rust UI Component System" }
                    ItemDescription { "Building a type-safe design system in Rust." }
                }
                ItemActions {
                    ChevronRight { class: "size-4 text-muted-foreground" }
                }
                ItemFooter {
                    Badge { variant: BadgeVariant::Secondary, "Design" }
                    span { class: "text-xs text-muted-foreground", "Mar 2026" }
                }
            }
        }
    }
}
