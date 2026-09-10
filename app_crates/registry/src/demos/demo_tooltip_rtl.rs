use dioxus::prelude::*;
use icons::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::tooltip::{Tooltip, TooltipContent, TooltipPosition, TooltipProvider};

#[component]
pub fn DemoTooltipRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            TooltipProvider {}

            div { class: "flex flex-col gap-4",
                Tooltip {
                    Button { variant: ButtonVariant::Secondary, ArrowUp {} }
                    TooltipContent { position: TooltipPosition::Top, "أعلى" }
                }
                Tooltip {
                    Button { variant: ButtonVariant::Secondary, ArrowLeft {} }
                    TooltipContent { position: TooltipPosition::Left, "يسار" }
                }
                Tooltip {
                    Button { variant: ButtonVariant::Secondary, ArrowRight {} }
                    TooltipContent { position: TooltipPosition::Right, "يمين" }
                }
                Tooltip {
                    Button { variant: ButtonVariant::Secondary, ArrowDown {} }
                    TooltipContent { position: TooltipPosition::Bottom, "أسفل" }
                }
            }
        }
    }
}
