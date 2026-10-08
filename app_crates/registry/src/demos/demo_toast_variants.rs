use dioxus::prelude::*;

use crate::ui::toast::{ToastTrigger, ToastType};

#[component]
pub fn DemoToastVariants() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2 justify-center",
            ToastTrigger { title: "You got a message", description: "You toasted me!", "Default" }
            ToastTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Success,
                "Success"
            }
            ToastTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Error,
                "Error"
            }
            ToastTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Warning,
                "Warning"
            }
            ToastTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Info,
                "Info"
            }
            ToastTrigger {
                title: "You got a message",
                description: "You toasted me!",
                variant: ToastType::Loading,
                "Loading"
            }
        }
    }
}
