use leptos::prelude::*;

use crate::ui::slider::{Slider, SliderVariant};

#[component]
pub fn DemoSliderVertical() -> impl IntoView {
    view! {
        <div class="flex gap-10 items-end">
            <div class="flex flex-col gap-3 items-center">
                <div class="flex justify-center items-center h-40">
                    <Slider class="w-40 -rotate-90" attr:value="60" />
                </div>
                <span class="text-sm text-muted-foreground">"Round"</span>
            </div>

            <div class="flex flex-col gap-3 items-center">
                <div class="flex justify-center items-center h-40">
                    <Slider variant=SliderVariant::Flat class="w-40 -rotate-90" attr:value="30" />
                </div>
                <span class="text-sm text-muted-foreground">"Flat"</span>
            </div>
        </div>
    }
}
