use dioxus::prelude::*;
use icons::{Mail, Search};

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};

#[component]
pub fn DemoInputGroupRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            div { class: "grid gap-6 w-full max-w-sm",
                InputGroup {
                    InputGroupInput { placeholder: "بحث..." }
                    InputGroupAddon {
                        Search { class: "size-4" }
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "أدخل بريدك الإلكتروني" }
                    InputGroupAddon {
                        Mail { class: "size-4" }
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "أدخل النص هنا" }
                    InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                        Search { class: "size-4" }
                    }
                }
            }
        }
    }
}
