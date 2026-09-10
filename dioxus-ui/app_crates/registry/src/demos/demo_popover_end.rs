use dioxus::prelude::*;

use crate::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverEnd() -> Element {
    rsx! {
        Popover { align: PopoverAlign::End,
            PopoverTrigger { "Popover (End)" }

            PopoverContent { class: "w-[300px]",
                PopoverTitle { "Popover End" }

                PopoverDescription {
                    "End-aligned popover that anchors to the right edge. Watch how it repositions intelligently as you scroll."
                }
            }
        }
    }
}
