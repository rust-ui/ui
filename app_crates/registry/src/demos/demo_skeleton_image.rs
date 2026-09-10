use dioxus::prelude::*;
use icons::Image;

use crate::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonImage() -> Element {
    rsx! {
        div { class: "m-4 space-y-8 w-full",
            div { class: "flex justify-center items-center w-full h-48 rounded-lg animate-pulse bg-muted",
                Image { class: "text-muted-foreground size-10" }
            }
            div { class: "space-y-2 w-full",
                Skeleton { class: "h-4" }
                Skeleton { class: "h-4 w-[80%]" }
                Skeleton { class: "h-4 w-[80%]" }
                Skeleton { class: "h-4 w-[80%]" }
            }
        }
    }
}
