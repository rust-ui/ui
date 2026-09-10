use dioxus::prelude::*;

use crate::ui::button_action::ButtonAction;

#[component]
pub fn DemoButtonAction() -> Element {
    rsx! {
        ButtonAction { on_complete: move |_| {}, "Hold to confirm" }
    }
}
