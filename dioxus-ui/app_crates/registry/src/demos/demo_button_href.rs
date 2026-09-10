use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoButtonHref() -> Element {
    rsx! {
        div { class: "flex gap-10 p-4 rounded-lg border",
            Button { href: "/", "Go to Home" }
            Button { href: "/", variant: ButtonVariant::Ghost, class: "border-2 border-dashed", "Go to Home (custom)" }
            Button { href: "https://dioxuslabs.com/", "External Link" }
        }
    }
}
