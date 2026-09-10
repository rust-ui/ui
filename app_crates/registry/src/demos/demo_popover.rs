use dioxus::prelude::*;

use crate::ui::popover::{Popover, PopoverContent, PopoverTrigger};

#[component]
pub fn DemoPopover() -> Element {
    rsx! {
        Popover {
            PopoverTrigger { "Open Popover" }
            PopoverContent {
                div { class: "flex flex-col gap-2",
                    p { class: "text-sm font-medium", "Popover" }
                    p { class: "text-sm text-muted-foreground",
                        "Click outside or press Escape to close."
                    }
                }
            }
        }
    }
}
