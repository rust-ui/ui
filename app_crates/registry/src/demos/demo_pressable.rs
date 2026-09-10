use dioxus::prelude::*;
use icons::Bell;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::card::Card;
use crate::ui::pressable::Pressable;

#[component]
pub fn DemoPressable() -> Element {
    rsx! {
        Pressable {
            Card { class: "flex gap-3 items-center py-3 px-4",
                div { class: "flex justify-center items-center rounded-full size-9 bg-primary/10",
                    Bell { class: "size-4 text-primary" }
                }
                div { class: "flex-1",
                    p { class: "text-sm font-medium", "New message received" }
                    p { class: "text-xs text-muted-foreground", "2 minutes ago" }
                }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Mark as read" }
            }
        }
    }
}
