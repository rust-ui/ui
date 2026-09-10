use dioxus::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerContent, DrawerDescription, DrawerHeader, DrawerPosition, DrawerTitle, DrawerTrigger,
};

#[component]
pub fn DemoDrawerSide() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }

            DrawerContent {
                position: DrawerPosition::Right,
                class: "top-0 bottom-0 left-auto h-full max-h-full rounded-t-none w-[300px] rounded-l-[10px]",

                DrawerBody {
                    DrawerHeader {
                        DrawerTitle { "Different Directions" }
                        DrawerDescription { "It supports all directions." }
                        DrawerDescription {
                            "This one specifically is not touching the edge of the screen, but that is not required for a side drawer."
                        }
                    }
                }
            }
        }
    }
}
