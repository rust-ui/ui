use leptos::prelude::*;

use crate::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverStartOuter() -> impl IntoView {
    view! {
        <Popover align=PopoverAlign::StartOuter>
            <PopoverTrigger>Popover (StartOuter)</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover StartOuter"</PopoverTitle>

                <PopoverDescription>
                    "StartOuter-aligned popover that positions completely to the left of the trigger button. The popover's right edge starts where the button's left edge ends."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
