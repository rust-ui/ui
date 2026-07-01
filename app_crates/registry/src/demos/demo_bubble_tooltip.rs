use icons::Check;
use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::tooltip::{Tooltip, TooltipContent, TooltipProvider};

#[component]
pub fn DemoBubbleTooltip() -> impl IntoView {
    view! {
        <TooltipProvider />
        <div class="flex flex-col gap-4 py-12 w-full max-w-sm">
            <Bubble variant=BubbleVariant::Secondary>
                <BubbleContent>"Did you remove the stale route?"</BubbleContent>
            </Bubble>
            <Bubble align=BubbleAlign::End>
                <BubbleContent>"Yes, removed it from the registry."</BubbleContent>
                <BubbleReactions>
                    // TODO PORT: shadcn uses TooltipTrigger render={<Button .../>} (asChild pattern).
                    // Ported as Button inside Tooltip — Tooltip wraps the trigger element directly.
                    <Tooltip>
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::IconXs>
                            <Check />
                        </Button>
                        <TooltipContent>"Read on Jan 5, 2026 at 4:32 PM"</TooltipContent>
                    </Tooltip>
                </BubbleReactions>
            </Bubble>
        </div>
    }
}
