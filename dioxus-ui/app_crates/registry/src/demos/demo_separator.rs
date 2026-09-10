use dioxus::prelude::*;

use crate::ui::separator::Separator;

#[component]
pub fn DemoSeparator() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 w-64",
            p { class: "text-sm", "Above the separator" }
            Separator {}
            p { class: "text-sm", "Below the separator" }
        }
    }
}
