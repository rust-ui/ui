use dioxus::prelude::*;
use icons::{BadgeCheck, ChevronRight};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemSize, ItemTitle, ItemVariant,
};

#[component]
pub fn DemoItem() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 w-full max-w-md",
            Item { variant: ItemVariant::Outline,
                ItemContent {
                    ItemTitle { "Basic Item" }
                    ItemDescription { "A simple item with title and description." }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Action" }
                }
            }
            Item { variant: ItemVariant::Outline, size: ItemSize::Sm, href: "#",
                ItemMedia { variant: ItemMediaVariant::Icon,
                    BadgeCheck { class: "size-5" }
                }
                ItemContent {
                    ItemTitle { "Your profile has been verified." }
                }
                ItemActions {
                    ChevronRight { class: "size-4" }
                }
            }
        }
    }
}
