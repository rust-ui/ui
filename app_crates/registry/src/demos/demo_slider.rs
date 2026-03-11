use leptos::prelude::*;

use crate::ui::slider::Slider;

#[component]
pub fn DemoSlider() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            <Slider />
            <Slider attr:min="0" attr:max="100" attr:value="40" attr:step="5" />
            <Slider attr:disabled="true" attr:value="80" />
        </div>
    }
}
