use leptos::prelude::*;

use crate::ui::popover::{Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopover() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>Open Popover</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover Demo"</PopoverTitle>

                <PopoverDescription>
                    "Interactive popover that adapts its position as you scroll. Try scrolling to see the smart positioning in action."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
