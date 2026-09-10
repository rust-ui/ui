use dioxus::prelude::*;

use crate::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonText() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 max-w-sm",
            Skeleton { class: "w-full h-4" }
            Skeleton { class: "h-4 w-[90%]" }
            Skeleton { class: "w-full h-4" }
            Skeleton { class: "h-4 w-[75%]" }
            Skeleton { class: "h-4 w-[85%]" }
            Skeleton { class: "h-4 w-[50%]" }
        }
    }
}
