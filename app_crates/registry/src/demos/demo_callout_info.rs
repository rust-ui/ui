use dioxus::prelude::*;

use crate::ui::callout::{Callout, CalloutVariant};

#[component]
pub fn DemoCalloutInfo() -> Element {
    rsx! {
        Callout { title: "Did you know?", variant: CalloutVariant::Info,
            "Components are server-side rendered by default. Use "
            code { "#[component]" }
            " to define reactive Dioxus components."
        }
    }
}
