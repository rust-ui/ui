use dioxus::prelude::*;

use crate::ui::aspect_ratio::AspectRatio;

#[component]
pub fn DemoAspectRatioSquare() -> Element {
    rsx! {
        div { class: "w-full max-w-[12rem]",
            AspectRatio { ratio: 1.0, class: "rounded-lg bg-muted",
                div { class: "bg-gradient-to-br from-emerald-400 to-teal-600 rounded-lg size-full" }
            }
        }
    }
}
