use dioxus::prelude::*;
use icons::Check;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::tooltip::{Tooltip, TooltipContent, TooltipProvider};

#[component]
pub fn DemoBubbleTooltip() -> Element {
    rsx! {
        TooltipProvider {}
        div { class: "flex flex-col gap-4 py-12 w-full max-w-sm",
            Bubble { variant: BubbleVariant::Secondary,
                BubbleContent { "Did you remove the stale route?" }
            }
            Bubble { align: BubbleAlign::End,
                BubbleContent { "Yes, removed it from the registry." }
                BubbleReactions {
                    Tooltip {
                        Button { variant: ButtonVariant::Ghost, size: ButtonSize::IconXs,
                            Check {}
                        }
                        TooltipContent { "Read on Jan 5, 2026 at 4:32 PM" }
                    }
                }
            }
        }
    }
}
