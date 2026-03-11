use leptos::prelude::*;

use crate::ui::slider::{Slider, SliderVariant};

#[component]
pub fn DemoSliderFlat() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            <Slider variant=SliderVariant::Flat />
            <Slider variant=SliderVariant::Flat attr:min="0" attr:max="100" attr:value="25" attr:step="5" />
            <Slider variant=SliderVariant::Flat attr:disabled="true" attr:value="64" />
        </div>
    }
}
