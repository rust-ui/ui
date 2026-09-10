use dioxus::prelude::*;

use crate::ui::slider::{Slider, SliderVariant};

#[component]
pub fn DemoSliderVertical() -> Element {
    rsx! {
        div { class: "flex gap-10 items-end",
            div { class: "flex flex-col gap-3 items-center",
                div { class: "flex justify-center items-center h-40",
                    Slider { class: "w-40 -rotate-90", value: 60.0 }
                }
                span { class: "text-sm text-muted-foreground", "Round" }
            }
            div { class: "flex flex-col gap-3 items-center",
                div { class: "flex justify-center items-center h-40",
                    Slider { variant: SliderVariant::Flat, class: "w-40 -rotate-90", value: 30.0 }
                }
                span { class: "text-sm text-muted-foreground", "Flat" }
            }
        }
    }
}
