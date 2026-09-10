use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::item::{Item, ItemActions, ItemContent, ItemDescription, ItemTitle, ItemVariant};

#[component]
pub fn DemoItemVariants() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            Item {
                ItemContent {
                    ItemTitle { "Default Variant" }
                    ItemDescription { "Standard styling with subtle background and borders." }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Open" }
                }
            }

            Item { variant: ItemVariant::Outline,
                ItemContent {
                    ItemTitle { "Outline Variant" }
                    ItemDescription { "Outlined style with clear borders and transparent background." }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Open" }
                }
            }

            Item { variant: ItemVariant::Muted,
                ItemContent {
                    ItemTitle { "Muted Variant" }
                    ItemDescription { "Subdued appearance with muted colors for secondary content." }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Open" }
                }
            }
        }
    }
}
