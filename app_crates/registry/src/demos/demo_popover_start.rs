use leptos::prelude::*;

use crate::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverStart() -> impl IntoView {
    view! {
        <Popover align=PopoverAlign::Start>
            <PopoverTrigger>Popover (Start)</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover Start"</PopoverTitle>

                <PopoverDescription>
                    "Start-aligned popover that anchors to the left edge. Watch how it repositions intelligently as you scroll."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
