use dioxus::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerContent, DrawerDescription, DrawerHeader, DrawerPosition, DrawerTitle, DrawerTrigger, DrawerVariant,
};

#[component]
pub fn DemoDrawerSideFloating() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }

            DrawerContent {
                position: DrawerPosition::Right,
                variant: DrawerVariant::Floating,
                class: "top-2 right-2 bottom-2 left-auto max-h-full outline-none w-[300px] rounded-[16px] rounded-t-[16px]",
                style: "--initial-transform: calc(100% + 8px);",

                DrawerHeader { class: "mt-4",
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
