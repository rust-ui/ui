use dioxus::prelude::*;
use icons::Search;

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};
use crate::ui::kbd::Kbd;

#[component]
pub fn DemoInputGroupKbd() -> Element {
    rsx! {
        InputGroup { class: "max-w-sm",
            InputGroupAddon {
                Search {}
            }
            InputGroupInput { placeholder: "Search..." }
            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                Kbd { "⌘K" }
            }
        }
    }
}
