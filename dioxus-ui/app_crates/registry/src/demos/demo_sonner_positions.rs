use dioxus::prelude::*;

use crate::ui::sonner::{SonnerPosition, SonnerToaster, SonnerTrigger};

#[component]
pub fn DemoSonnerPositions() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2 justify-center",
            SonnerTrigger {
                title: "Top Left",
                description: "Toast positioned at top-left",
                position: "TopLeft",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Top Left"
            }
            SonnerTrigger {
                title: "Top Center",
                description: "Toast positioned at top-center",
                position: "TopCenter",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Top Center"
            }
            SonnerTrigger {
                title: "Top Right",
                description: "Toast positioned at top-right",
                position: "TopRight",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Top Right"
            }
            SonnerTrigger {
                title: "Bottom Left",
                description: "Toast positioned at bottom-left",
                position: "BottomLeft",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Bottom Left"
            }
            SonnerTrigger {
                title: "Bottom Center",
                description: "Toast positioned at bottom-center",
                position: "BottomCenter",
                class: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                "Bottom Center"
            }
            SonnerTrigger {
                title: "Bottom Right",
                description: "Toast positioned at bottom-right (default)",
                "Bottom Right (Default)"
            }
        }

        // Toasters at each position
        SonnerToaster { position: SonnerPosition::TopLeft }
        SonnerToaster { position: SonnerPosition::TopCenter }
        SonnerToaster { position: SonnerPosition::TopRight }
        SonnerToaster { position: SonnerPosition::BottomLeft }
        SonnerToaster { position: SonnerPosition::BottomCenter }
    }
}
