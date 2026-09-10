use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::slider::Slider;

#[component]
pub fn DemoSliderRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            div { class: "flex flex-col gap-6 w-full max-w-sm",
                Slider {}
                Slider { min: 0.0, max: 100.0, value: 40.0, step: 5.0 }
                Slider { disabled: true, value: 80.0 }
            }
        }
    }
}
