use dioxus::prelude::*;

use crate::hooks::use_random::use_random_id;

#[component]
pub fn DemoUseRandom() -> Element {
    let checkbox_id = use_random_id();

    rsx! {
        div { class: "p-4 mx-auto space-y-4 max-w-md",
            h3 { class: "font-semibold", "Random ID Hook" }
            p { class: "text-sm",
                "Generated ID: "
                code { class: "py-1 px-2 text-xs rounded bg-muted", "{checkbox_id.clone()}" }
            }
            div { class: "flex gap-2 items-center",
                input { r#type: "checkbox", id: "{checkbox_id}" }
                label { class: "text-sm", "Checkbox with unique ID" }
            }
        }
    }
}
