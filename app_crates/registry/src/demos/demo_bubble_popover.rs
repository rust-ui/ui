use icons::Info;
use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::popover::{Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoBubblePopover() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 py-12 w-full max-w-sm">
            <Bubble align=BubbleAlign::End>
                <BubbleContent>"Run the build script."</BubbleContent>
            </Bubble>
            <Bubble variant=BubbleVariant::Destructive>
                <BubbleContent>"Failed to run the command."</BubbleContent>
                <BubbleReactions>
                    // TODO PORT: shadcn uses PopoverTrigger render={<Button .../>} (asChild pattern).
                    // Ported as Button inside PopoverTrigger.
                    <Popover>
                        <PopoverTrigger>
                            <Button
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconXs
                                attr:aria-label="Show error details"
                                class="aria-expanded:text-destructive"
                            >
                                <Info />
                            </Button>
                        </PopoverTrigger>
                        <PopoverContent>
                            <PopoverTitle class="text-sm">"Command failed with exit code 1"</PopoverTitle>
                            <PopoverDescription class="text-sm">
                                "ENOENT: no such file or directory, open pnpm-lock.yaml"
                            </PopoverDescription>
                        </PopoverContent>
                    </Popover>
                </BubbleReactions>
            </Bubble>
        </div>
    }
}
