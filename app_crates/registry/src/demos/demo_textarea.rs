use dioxus::prelude::*;

use crate::ui::textarea::Textarea;

#[component]
pub fn DemoTextarea() -> Element {
    rsx! {
        Textarea { placeholder: "Type your message here..." }
    }
}
