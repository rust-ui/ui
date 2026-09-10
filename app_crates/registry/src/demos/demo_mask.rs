use dioxus::prelude::*;

use crate::ui::mask::{Mask, MaskSide, MaskWrapper};

#[component]
pub fn DemoMask() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 w-full max-w-md",
            MaskWrapper {
                div { class: "flex gap-4 items-center",
                    div { class: "w-48 h-20 rounded-md bg-primary/20 flex items-center justify-center text-sm", "Content" }
                    div { class: "w-48 h-20 rounded-md bg-primary/20 flex items-center justify-center text-sm", "Content" }
                    div { class: "w-48 h-20 rounded-md bg-primary/20 flex items-center justify-center text-sm", "Content" }
                }
                Mask { side: MaskSide::Left }
                Mask { side: MaskSide::Right }
            }
        }
    }
}
