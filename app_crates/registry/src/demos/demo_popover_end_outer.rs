use leptos::prelude::*;

use crate::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverEndOuter() -> impl IntoView {
    view! {
        <Popover align=PopoverAlign::EndOuter>
            <PopoverTrigger>Popover (EndOuter)</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover EndOuter"</PopoverTitle>

                <PopoverDescription>
                    "EndOuter-aligned popover that positions completely to the right of the trigger button. The popover's left edge starts where the button's right edge ends."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
