use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::slider::Slider;

#[component]
pub fn DemoSliderRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-6 w-full max-w-sm">
                <Slider />
                <Slider attr:min="0" attr:max="100" attr:value="40" attr:step="5" />
                <Slider attr:disabled="true" attr:value="80" />
            </div>
        </DirectionProvider>
    }
}
