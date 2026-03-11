use leptos::prelude::*;

use crate::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverEnd() -> impl IntoView {
    view! {
        <Popover align=PopoverAlign::End>
            <PopoverTrigger>Popover (End)</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover End"</PopoverTitle>

                <PopoverDescription>
                    "End-aligned popover that anchors to the right edge. Notice the smart positioning behavior during scroll."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
