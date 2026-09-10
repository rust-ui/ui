use dioxus::prelude::*;

use crate::ui::progress::Progress;

#[component]
pub fn DemoProgress() -> Element {
    rsx! {
        div { class: "w-full max-w-sm",
            Progress { value: 60.0 }
        }
    }
}
