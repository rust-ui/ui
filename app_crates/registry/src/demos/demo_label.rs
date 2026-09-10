use dioxus::prelude::*;

use crate::ui::label::Label;

#[component]
pub fn DemoLabel() -> Element {
    rsx! {
        Label { html_for: "email", "Email address" }
    }
}
