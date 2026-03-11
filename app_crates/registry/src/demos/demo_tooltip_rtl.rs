use icons::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::tooltip::{Tooltip, TooltipContent, TooltipPosition, TooltipProvider};

#[component]
pub fn DemoTooltipRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <TooltipProvider />

            <div class="flex flex-col gap-4">
                <Tooltip>
                    <Button variant=ButtonVariant::Secondary>
                        <ArrowUp />
                    </Button>
                    <TooltipContent position=TooltipPosition::Top>"أعلى"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <Button variant=ButtonVariant::Secondary>
                        <ArrowLeft />
                    </Button>
                    <TooltipContent position=TooltipPosition::Left>"يسار"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <Button variant=ButtonVariant::Secondary>
                        <ArrowRight />
                    </Button>
                    <TooltipContent position=TooltipPosition::Right>"يمين"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <Button variant=ButtonVariant::Secondary>
                        <ArrowDown />
                    </Button>
                    <TooltipContent position=TooltipPosition::Bottom>"أسفل"</TooltipContent>
                </Tooltip>
            </div>
        </DirectionProvider>
    }
}
