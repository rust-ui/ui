use dioxus::prelude::*;

use crate::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverStart() -> Element {
    rsx! {
        Popover { align: PopoverAlign::Start,
            PopoverTrigger { "Popover (Start)" }

            PopoverContent { class: "w-[300px]",
                PopoverTitle { "Popover Start" }

                PopoverDescription {
                    "Start-aligned popover that anchors to the left edge. Watch how it repositions intelligently as you scroll."
                }
            }
        }
    }
}
