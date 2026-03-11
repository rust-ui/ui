use icons::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Link};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::tooltip::{Tooltip, TooltipContent, TooltipPosition, TooltipProvider};

#[component]
pub fn DemoTooltip() -> impl IntoView {
    view! {
        <TooltipProvider />

        <div class="flex flex-col gap-4">
            <Tooltip>
                <Button variant=ButtonVariant::Secondary>
                    <ArrowUp />
                </Button>
                <TooltipContent position=TooltipPosition::Top>"TOP"</TooltipContent>
            </Tooltip>
            <Tooltip>
                <Button variant=ButtonVariant::Secondary>
                    <ArrowLeft />
                </Button>
                <TooltipContent position=TooltipPosition::Left>"LEFT"</TooltipContent>
            </Tooltip>
            <Tooltip>
                <Button variant=ButtonVariant::Secondary>
                    <ArrowRight />
                </Button>
                <TooltipContent position=TooltipPosition::Right>"RIGHT"</TooltipContent>
            </Tooltip>
            <Tooltip>
                <Button variant=ButtonVariant::Secondary>
                    <ArrowDown />
                </Button>
                <TooltipContent position=TooltipPosition::Bottom>"BOTTOM"</TooltipContent>
            </Tooltip>

            <Tooltip>
                <Button variant=ButtonVariant::Secondary attr:href="https://rust-ui.com">
                    <Link />
                </Button>
                <TooltipContent position=TooltipPosition::Right>"Link to Rust/UI"</TooltipContent>
            </Tooltip>
        </div>
    }
}
