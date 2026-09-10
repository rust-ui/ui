use dioxus::prelude::*;

use crate::ui::callout::{Callout, CalloutVariant};

#[component]
pub fn DemoCalloutWarning() -> Element {
    rsx! {
        Callout { title: "Breaking Change", variant: CalloutVariant::Warning,
            "This API changed in v2.0. Update your imports from "
            code { "dioxus_ui::old" }
            " to "
            code { "dioxus_ui::ui" }
            "."
        }
    }
}
