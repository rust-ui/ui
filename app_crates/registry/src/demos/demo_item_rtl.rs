use dioxus::prelude::*;
use icons::{BadgeCheck, ChevronRight};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::item::{Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemSize, ItemTitle, ItemVariant};

#[component]
pub fn DemoItemRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-md",
            div { class: "flex flex-col gap-6 w-full max-w-md",
                Item { variant: ItemVariant::Outline,
                    ItemContent {
                        ItemTitle { "عنصر أساسي" }
                        ItemDescription { "عنصر بسيط يحتوي على عنوان ووصف." }
                    }
                    ItemActions {
                        Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "إجراء" }
                    }
                }

                Item { variant: ItemVariant::Outline, size: ItemSize::Sm, href: "#",
                    ItemMedia {
                        BadgeCheck { class: "size-5" }
                    }
                    ItemContent {
                        ItemTitle { "تم التحقق من ملفك الشخصي." }
                    }
                    ItemActions {
                        ChevronRight { class: "size-4 rtl:rotate-180" }
                    }
                }
            }
        }
    }
}
