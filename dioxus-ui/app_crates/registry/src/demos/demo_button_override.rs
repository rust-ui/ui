use dioxus::prelude::*;

use crate::ui::button::Button;

#[component]
pub fn DemoButtonOverride() -> Element {
    rsx! {
        Button { class: "hover:bg-pink-500 bg-sky-500", "Fancy Button" }
    }
}
