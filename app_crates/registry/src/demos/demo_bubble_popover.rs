use dioxus::prelude::*;
use icons::Info;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleVariant};
use crate::ui::popover::{Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoBubblePopover() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 py-12 w-full max-w-sm",
            Bubble { align: BubbleAlign::End,
                BubbleContent { "Run the build script." }
            }
            Bubble { variant: BubbleVariant::Destructive,
                BubbleContent { "Failed to run the command." }
                BubbleReactions {
                    Popover {
                        PopoverTrigger {
                            aria_label: "Show error details",
                            class: "size-6 rounded-md border-none bg-transparent p-0 shadow-none hover:bg-accent hover:text-accent-foreground aria-expanded:text-destructive",
                            Info {}
                        }
                        PopoverContent {
                            PopoverTitle { class: "text-sm", "Command failed with exit code 1" }
                            PopoverDescription { class: "text-sm",
                                "ENOENT: no such file or directory, open pnpm-lock.yaml"
                            }
                        }
                    }
                }
            }
        }
    }
}
