use dioxus::prelude::*;

use crate::ui::button::Button;

#[component]
pub fn DemoButtonDisabled() -> Element {
    rsx! {
        div { class: "flex gap-4",
            Button { "Normal" }
            Button { disabled: true, "Disabled" }
        }
    }
}
