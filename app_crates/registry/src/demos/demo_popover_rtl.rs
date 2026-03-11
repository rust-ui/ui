use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::popover::{Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Popover>
                <PopoverTrigger>"فتح النافذة المنبثقة"</PopoverTrigger>

                <PopoverContent class="w-[300px]">
                    <PopoverTitle>"نافذة منبثقة تجريبية"</PopoverTitle>

                    <PopoverDescription>
                        "نافذة منبثقة تفاعلية تتكيف موضعها أثناء التمرير. جرّب التمرير لرؤية التمركز الذكي."
                    </PopoverDescription>
                </PopoverContent>
            </Popover>
        </DirectionProvider>
    }
}
