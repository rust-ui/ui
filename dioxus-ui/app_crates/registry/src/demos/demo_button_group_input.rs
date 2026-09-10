use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::button_group::ButtonGroup;
use crate::ui::input::Input;

#[component]
pub fn DemoButtonGroupInput() -> Element {
    rsx! {
        ButtonGroup {
            Input { class: "w-64", placeholder: "Enter a URL" }
            Button { "Subscribe" }
        }
    }
}
