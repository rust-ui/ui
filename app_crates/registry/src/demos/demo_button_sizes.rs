use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonSize};

#[component]
pub fn DemoButtonSizes() -> Element {
    rsx! {
        div { class: "flex gap-4 items-center",
            Button { size: ButtonSize::Sm, "Small" }
            Button { "Default" }
            Button { size: ButtonSize::Lg, "Large" }
        }
    }
}
