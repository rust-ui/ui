use icons::ChevronsUpDown;
use leptos::prelude::*;

use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoCollapsibleRtl() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Collapsible open=open class="flex flex-col gap-2 w-[350px]">
                <div class="flex gap-4 justify-between items-center px-4">
                    <h4 class="text-sm font-semibold">"طلب رقم #٤١٨٩"</h4>
                    <CollapsibleTrigger class="inline-flex justify-center items-center rounded-md transition-colors size-8 hover:bg-accent hover:text-accent-foreground">
                        <ChevronsUpDown class="size-4" />
                        <span class="sr-only">"تبديل التفاصيل"</span>
                    </CollapsibleTrigger>
                </div>
                <div class="flex justify-between items-center py-2 px-4 text-sm rounded-md border">
                    <span class="text-muted-foreground">"الحالة"</span>
                    <span class="font-medium">"تم الشحن"</span>
                </div>
                <CollapsibleContent class="flex flex-col gap-2">
                    <div class="py-2 px-4 text-sm rounded-md border">
                        <p class="font-medium">"عنوان الشحن"</p>
                        <p class="text-muted-foreground">"١٠٠ شارع السوق، الرياض"</p>
                    </div>
                    <div class="py-2 px-4 text-sm rounded-md border">
                        <p class="font-medium">"العناصر"</p>
                        <p class="text-muted-foreground">"٢x سماعات استوديو"</p>
                    </div>
                </CollapsibleContent>
            </Collapsible>
        </DirectionProvider>
    }
}
