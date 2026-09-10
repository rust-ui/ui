use dioxus::prelude::*;

use crate::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeleton() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 w-64",
            Skeleton { class: "h-4 w-full" }
            Skeleton { class: "h-4 w-3/4" }
            Skeleton { class: "h-4 w-1/2" }
            Skeleton { class: "h-10 w-full rounded-xl" }
        }
    }
}
