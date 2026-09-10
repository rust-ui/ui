use dioxus::prelude::*;

use crate::ui::button::Button;

#[component]
pub fn DemoButton() -> Element {
    rsx! {
        Button { "Button" }
    }
}
