use dioxus::prelude::*;

use crate::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
pub fn DemoAlert() -> Element {
    rsx! {
        Alert {
            AlertTitle { "Heads up!" }
            AlertDescription { "You can add components to your app using the CLI." }
        }
    }
}
