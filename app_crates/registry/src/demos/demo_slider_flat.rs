use dioxus::prelude::*;

use crate::ui::slider::{Slider, SliderVariant};

#[component]
pub fn DemoSliderFlat() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            Slider { variant: SliderVariant::Flat }
            Slider { variant: SliderVariant::Flat, min: 0.0, max: 100.0, value: 25.0, step: 5.0 }
            Slider { variant: SliderVariant::Flat, disabled: true, value: 64.0 }
        }
    }
}
