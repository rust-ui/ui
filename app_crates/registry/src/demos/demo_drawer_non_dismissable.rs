use dioxus::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerNonDismissable() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }

            DrawerContent {
                style: "--initial-transform: 100%; pointer-events: auto;",
                dismissible: false,

                DrawerHandle {}

                DrawerBody {
                    DrawerHeader {
                        DrawerTitle { "Are you absolutely sure?" }
                        DrawerDescription {
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }

                    DrawerClose { "Confirm" }
                }
            }
        }
    }
}
