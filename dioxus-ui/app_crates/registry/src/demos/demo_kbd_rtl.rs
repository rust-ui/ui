use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::kbd::{Kbd, KbdGroup};

#[component]
pub fn DemoKbdRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            div { class: "flex flex-col gap-4 items-center",
                div { class: "flex gap-2 items-center text-sm text-muted-foreground",
                    span { "بحث:" }
                    KbdGroup {
                        Kbd { "⌘" }
                        Kbd { "K" }
                    }
                }
                div { class: "flex gap-2 items-center text-sm text-muted-foreground",
                    span { "غامق:" }
                    KbdGroup {
                        Kbd { "Ctrl" }
                        span { "+" }
                        Kbd { "B" }
                    }
                }
            }
        }
    }
}
