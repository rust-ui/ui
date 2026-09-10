use dioxus::prelude::*;

use crate::ui::status::{Status, StatusVariant};

#[component]
pub fn DemoStatusVariants() -> Element {
    rsx! {
        div { class: "flex gap-6 items-center flex-wrap",
            Status { variant: StatusVariant::Normal,
                div { class: "rounded-md size-16 bg-neutral-500" }
            }
            Status { variant: StatusVariant::Active,
                div { class: "rounded-md size-16 bg-neutral-500" }
            }
            Status { variant: StatusVariant::Inactive,
                div { class: "rounded-md size-16 bg-neutral-500" }
            }
            Status { variant: StatusVariant::Default,
                div { class: "rounded-md size-16 bg-neutral-500" }
            }
        }
    }
}
