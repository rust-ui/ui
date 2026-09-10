use dioxus::prelude::*;
use icons::ChevronRight;

use crate::ui::avatar::{Avatar, AvatarFallback};
use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemHeader, ItemMedia, ItemSeparator, ItemTitle,
};

#[component]
pub fn DemoItemGroup() -> Element {
    rsx! {
        ItemGroup { class: "w-full max-w-md rounded-md border",
            Item { class: "py-3 px-4 rounded-none",
                ItemHeader {
                    Badge { variant: BadgeVariant::Secondary, "Message" }
                    span { class: "text-xs text-muted-foreground", "2m ago" }
                }
                ItemMedia {
                    Avatar { class: "size-8",
                        AvatarFallback { "RS" }
                    }
                }
                ItemContent {
                    ItemTitle { "Ryan Smith sent a message" }
                    ItemDescription { "Hey, are you free for a quick call?" }
                }
                ItemActions {
                    ChevronRight { class: "size-4 text-muted-foreground" }
                }
            }
            ItemSeparator {}
            Item { class: "py-3 px-4 rounded-none",
                ItemHeader {
                    Badge { variant: BadgeVariant::Secondary, "Alert" }
                    span { class: "text-xs text-muted-foreground", "15m ago" }
                }
                ItemMedia {
                    Avatar { class: "size-8",
                        AvatarFallback { "SY" }
                    }
                }
                ItemContent {
                    ItemTitle { "Deploy completed" }
                    ItemDescription { "Production build succeeded with no errors." }
                }
                ItemActions {
                    ChevronRight { class: "size-4 text-muted-foreground" }
                }
            }
            ItemSeparator {}
            Item { class: "py-3 px-4 rounded-none",
                ItemHeader {
                    Badge { variant: BadgeVariant::Secondary, "Update" }
                    span { class: "text-xs text-muted-foreground", "1h ago" }
                }
                ItemMedia {
                    Avatar { class: "size-8",
                        AvatarFallback { "MK" }
                    }
                }
                ItemContent {
                    ItemTitle { "Morgan updated the docs" }
                    ItemDescription { "Installation guide revised for Dioxus." }
                }
                ItemActions {
                    ChevronRight { class: "size-4 text-muted-foreground" }
                }
            }
        }
    }
}
