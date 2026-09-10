use dioxus::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerContent, DrawerDescription, DrawerHeader, DrawerPosition, DrawerTitle, DrawerTrigger,
};

#[component]
pub fn DemoDrawerSideScrollable() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }

            DrawerContent {
                position: DrawerPosition::Left,
                class: "top-0 bottom-0 right-auto h-full max-h-full rounded-t-none w-[300px] rounded-r-[10px]",

                DrawerBody { class: "overflow-y-auto pr-4 h-full",
                    DrawerHeader {
                        DrawerTitle { "Scrollable Side Drawer" }
                        DrawerDescription {
                            "This drawer contains 50 scrollable items to demonstrate side scrolling behavior."
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
                }
            }
        }
    }
}
