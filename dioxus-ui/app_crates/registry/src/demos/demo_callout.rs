use dioxus::prelude::*;

use crate::ui::callout::Callout;

#[component]
pub fn DemoCallout() -> Element {
    rsx! {
        Callout { title: "Note", "You can add components and dependencies to your app using the CLI." }
    }
}
