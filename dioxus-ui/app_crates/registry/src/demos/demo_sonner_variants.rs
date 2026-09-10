use dioxus::prelude::*;

use crate::ui::sonner::{SonnerTrigger, ToastType};

#[component]
pub fn DemoSonnerVariants() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2 justify-center",
            SonnerTrigger { title: "You got a message", description: "You toasted me!", "Default" }
            SonnerTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Success,
                "Success"
            }
            SonnerTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Error,
                "Error"
            }
            SonnerTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Warning,
                "Warning"
            }
            SonnerTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Info,
                "Info"
            }
            SonnerTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Loading,
                "Loading"
            }
        }
    }
}
