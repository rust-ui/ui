use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::tooltip::{Tooltip, TooltipContent};

#[component]
pub fn DemoTooltip() -> Element {
    rsx! {
        div { class: "flex gap-8 items-center justify-center flex-wrap",
            Tooltip {
                Button { "Hover me" }
                TooltipContent { "This is a tooltip" }
            }
            Tooltip {
                Button { variant: crate::ui::button::ButtonVariant::Outline, "Right side" }
                TooltipContent {
                    position: crate::ui::tooltip::TooltipPosition::Right,
                    "Tooltip on the right"
                }
            }
            Tooltip {
                Button { variant: crate::ui::button::ButtonVariant::Secondary, "Bottom" }
                TooltipContent {
                    position: crate::ui::tooltip::TooltipPosition::Bottom,
                    "Tooltip on the bottom"
                }
            }
        }
    }
}
