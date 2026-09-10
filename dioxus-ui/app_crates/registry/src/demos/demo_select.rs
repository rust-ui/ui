use dioxus::prelude::*;

use crate::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const OPTIONS: [&str; 3] = ["Components", "Extensions", "Icons"];

#[component]
pub fn DemoSelect() -> Element {
    rsx! {
        Select {
            SelectTrigger { class: "w-[150px]",
                SelectValue { placeholder: "Please select" }
            }

            SelectContent {
                SelectGroup {
                    for option in OPTIONS {
                        SelectOption { value: option, "{option}" }
                    }
                }
            }
        }
    }
}
