use dioxus::prelude::*;

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput, InputGroupText};

#[component]
pub fn DemoInputGroupText() -> Element {
    rsx! {
        div { class: "grid gap-6 w-full max-w-sm",
            InputGroup {
                InputGroupAddon {
                    InputGroupText { "$" }
                }
                InputGroupInput { placeholder: "0.00" }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    InputGroupText { "USD" }
                }
            }
            InputGroup {
                InputGroupAddon {
                    InputGroupText { "https://" }
                }
                InputGroupInput { placeholder: "example.com", class: "!pl-0.5" }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    InputGroupText { ".com" }
                }
            }
            InputGroup {
                InputGroupInput { placeholder: "Enter your username" }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    InputGroupText { "@company.com" }
                }
            }
        }
    }
}
