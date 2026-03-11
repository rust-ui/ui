use leptos::prelude::*;

use crate::ui::slider::Slider;

#[component]
pub fn DemoSliderMultiple() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-5 w-80">
            <div class="flex flex-col gap-1.5">
                <div class="flex justify-between text-sm">
                    <span class="text-muted-foreground">"Low"</span>
                    <span class="font-medium tabular-nums">"10"</span>
                </div>
                <Slider attr:value="10" />
            </div>

            <div class="flex flex-col gap-1.5">
                <div class="flex justify-between text-sm">
                    <span class="text-muted-foreground">"Mid"</span>
                    <span class="font-medium tabular-nums">"20"</span>
                </div>
                <Slider attr:value="20" />
            </div>

            <div class="flex flex-col gap-1.5">
                <div class="flex justify-between text-sm">
                    <span class="text-muted-foreground">"High"</span>
                    <span class="font-medium tabular-nums">"70"</span>
                </div>
                <Slider attr:value="70" />
            </div>
        </div>
    }
}
