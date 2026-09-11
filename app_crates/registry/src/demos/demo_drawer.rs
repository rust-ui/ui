use dioxus::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawer() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }
            DrawerContent {
                DrawerHandle {}
                DrawerBody {
                    DrawerHeader {
                        DrawerTitle { "Drawer Title" }
                        DrawerDescription { "Drag down to close or click outside." }
                    }
                    DrawerClose { "Close" }
                }
            }
        }
    }
}
