use dioxus::prelude::*;

use crate::ui::sheet::{
    Sheet, SheetBody, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetTitle, SheetTrigger,
};

#[component]
pub fn DemoSheetDirections() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 items-center",
            DemoSheetTop {}
            div { class: "flex gap-4",
                DemoSheetLeft {}
                DemoSheetRight {}
            }
            DemoSheetBottom {}
        }
    }
}

#[component]
pub fn DemoSheetTop() -> Element {
    rsx! {
        Sheet {
            SheetTrigger { "Top" }
            SheetContent { direction: SheetDirection::Top,
                SheetBody {
                    SheetTitle { "Top Sheet" }
                    SheetDescription { "This sheet slides from the top." }
                    SheetClose { "Close" }
                }
            }
        }
    }
}

#[component]
pub fn DemoSheetLeft() -> Element {
    rsx! {
        Sheet {
            SheetTrigger { "Left" }
            SheetContent { direction: SheetDirection::Left,
                SheetBody {
                    SheetTitle { "Left Sheet" }
                    SheetDescription { "This sheet slides from the left." }
                    SheetClose { "Close" }
                }
            }
        }
    }
}

#[component]
pub fn DemoSheetRight() -> Element {
    rsx! {
        Sheet {
            SheetTrigger { "Right" }
            SheetContent { direction: SheetDirection::Right,
                SheetBody {
                    SheetTitle { "Right Sheet" }
                    SheetDescription { "This sheet slides from the right." }
                    SheetClose { "Close" }
                }
            }
        }
    }
}

#[component]
pub fn DemoSheetBottom() -> Element {
    rsx! {
        Sheet {
            SheetTrigger { "Bottom" }
            SheetContent { direction: SheetDirection::Bottom,
                SheetBody {
                    SheetTitle { "Bottom Sheet" }
                    SheetDescription { "This sheet slides from the bottom." }
                    SheetClose { "Close" }
                }
            }
        }
    }
}
