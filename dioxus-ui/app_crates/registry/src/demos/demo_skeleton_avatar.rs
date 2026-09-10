use dioxus::prelude::*;

use crate::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonAvatar() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            div { class: "flex gap-3 items-center",
                Skeleton { class: "rounded-full size-10" }
                div { class: "flex flex-col gap-2",
                    Skeleton { class: "w-32 h-4" }
                    Skeleton { class: "w-24 h-3" }
                }
            }
            div { class: "flex gap-3 items-center",
                Skeleton { class: "rounded-full size-10" }
                div { class: "flex flex-col gap-2",
                    Skeleton { class: "w-40 h-4" }
                    Skeleton { class: "w-28 h-3" }
                }
            }
            div { class: "flex gap-3 items-center",
                Skeleton { class: "rounded-full size-10" }
                div { class: "flex flex-col gap-2",
                    Skeleton { class: "w-36 h-4" }
                    Skeleton { class: "w-20 h-3" }
                }
            }
        }
    }
}
