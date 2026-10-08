use dioxus::prelude::*;

use crate::ui::toast::{ToastPosition, ToastToaster, ToastTrigger};

#[component]
pub fn DemoToastPositions() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2 justify-center",
            ToastTrigger {
                title: "Top Left",
                description: "Toast positioned at top-left",
                position: "TopLeft",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Top Left"
            }
            ToastTrigger {
                title: "Top Center",
                description: "Toast positioned at top-center",
                position: "TopCenter",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Top Center"
            }
            ToastTrigger {
                title: "Top Right",
                description: "Toast positioned at top-right",
                position: "TopRight",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Top Right"
            }
            ToastTrigger {
                title: "Bottom Left",
                description: "Toast positioned at bottom-left",
                position: "BottomLeft",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Bottom Left"
            }
            ToastTrigger {
                title: "Bottom Center",
                description: "Toast positioned at bottom-center",
                position: "BottomCenter",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Bottom Center"
            }
            ToastTrigger {
                title: "Bottom Right",
                description: "Toast positioned at bottom-right (default)",
                "Bottom Right (Default)"
            }
        }

        // Toasters at each position
        ToastToaster { position: ToastPosition::TopLeft }
        ToastToaster { position: ToastPosition::TopCenter }
        ToastToaster { position: ToastPosition::TopRight }
        ToastToaster { position: ToastPosition::BottomLeft }
        ToastToaster { position: ToastPosition::BottomCenter }
    }
}
