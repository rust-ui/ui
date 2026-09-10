use dioxus::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerScrollable() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }

            DrawerContent {
                DrawerHandle {}

                DrawerBody { class: "overflow-y-auto",
                    DrawerHeader {
                        DrawerTitle { "Scrollable Drawer" }
                        DrawerDescription {
                            "This drawer contains many items to demonstrate scrolling behavior."
                        }
                    }

                    div { class: "mt-4 space-y-2",
                        for i in 1..=50 {
                            div { class: "p-3 rounded-md border bg-muted border-border",
                                p { class: "text-sm font-medium", "Item {i}" }
                                p { class: "text-xs text-muted-foreground",
                                    "This is item number {i} in the list"
                                }
                            }
                        }
                    }

                    DrawerClose { "Close" }
                }
            }
        }
    }
}
