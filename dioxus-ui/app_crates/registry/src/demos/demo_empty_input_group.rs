use dioxus::prelude::*;
use icons::Search;

use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle};
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};
use crate::ui::kbd::Kbd;

#[component]
pub fn DemoEmptyInputGroup() -> Element {
    rsx! {
        Empty {
            EmptyHeader {
                EmptyTitle { "404 - Not Found" }
                EmptyDescription {
                    "The page you're looking for doesn't exist. Try searching for what you need below."
                }
            }
            EmptyContent { class: "flex-col",
                InputGroup { class: "sm:w-3/4",
                    InputGroupInput { placeholder: "Try searching for pages..." }
                    InputGroupAddon {
                        Search {}
                    }
                    InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                        Kbd { "/" }
                    }
                }
                EmptyDescription {
                    "Need help? "
                    a { href: "#", class: "underline transition-colors underline-offset-4 hover:text-foreground",
                        "Contact support"
                    }
                }
            }
        }
    }
}
