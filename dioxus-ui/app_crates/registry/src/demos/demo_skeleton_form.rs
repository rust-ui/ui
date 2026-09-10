use dioxus::prelude::*;

use crate::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonForm() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 max-w-sm",
            div { class: "flex flex-col gap-2",
                Skeleton { class: "w-20 h-4" }
                Skeleton { class: "w-full h-9 rounded-md" }
            }
            div { class: "flex flex-col gap-2",
                Skeleton { class: "w-16 h-4" }
                Skeleton { class: "w-full h-9 rounded-md" }
            }
            div { class: "flex flex-col gap-2",
                Skeleton { class: "w-24 h-4" }
                Skeleton { class: "w-full h-9 rounded-md" }
            }
            Skeleton { class: "w-full h-9 rounded-md" }
        }
    }
}
