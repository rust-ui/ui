use dioxus::prelude::*;
use icons::{Check, CreditCard, Info, Mail, Search, Star};

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};

#[component]
pub fn DemoInputGroup() -> Element {
    rsx! {
        div { class: "grid gap-6 w-full max-w-sm",
            InputGroup {
                InputGroupInput { placeholder: "Search..." }
                InputGroupAddon {
                    Search { class: "size-4" }
                }
            }
            InputGroup {
                InputGroupInput { placeholder: "Enter your email" }
                InputGroupAddon {
                    Mail { class: "size-4" }
                }
            }
            InputGroup {
                InputGroupInput { placeholder: "Card number" }
                InputGroupAddon {
                    CreditCard { class: "size-4" }
                }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    Check { class: "size-4" }
                }
            }
            InputGroup {
                InputGroupInput { placeholder: "Card number" }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    Star { class: "size-4" }
                    Info { class: "size-4" }
                }
            }
        }
    }
}
