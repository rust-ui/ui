use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHandle, DrawerHeader,
    DrawerTitle, DrawerTrigger,
};

#[component]
pub fn DemoDrawer() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }
            DrawerContent {
                DrawerHandle {}
                DrawerHeader {
                    DrawerTitle { "Are you sure?" }
                    DrawerDescription { "This action cannot be undone." }
                }
                DrawerBody {
                    p { class: "text-sm text-muted-foreground",
                        "Drawer content goes here. Swipe down or click outside to close."
                    }
                }
                DrawerFooter {
                    DrawerClose { "Cancel" }
                    Button { "Continue" }
                }
            }
        }
    }
}
