use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoButtonVariants() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4 items-center",
            Button { "Default" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
            Button { variant: ButtonVariant::Destructive, "Destructive" }
            Button { variant: ButtonVariant::Link, "Link" }
        }
    }
}
