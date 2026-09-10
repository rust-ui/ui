use dioxus::prelude::*;
use icons::Info;

use crate::ui::field::{Field, FieldGroup, FieldLabel};
use crate::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize, InputGroupInput,
};
use crate::ui::tooltip::{Tooltip, TooltipContent};

#[component]
pub fn DemoInputGroupTooltip() -> Element {
    rsx! {
        FieldGroup { class: "max-w-sm",
            Field {
                FieldLabel { r#for: "email-tooltip", "Email" }
                InputGroup {
                    InputGroupInput { id: "email-tooltip", placeholder: "you@example.com" }
                }
            }

            Field {
                FieldLabel {
                    div { class: "flex justify-between items-center w-full",
                        "Username"
                        Tooltip {
                            InputGroupButton { size: InputGroupButtonSize::IconXs, class: "rounded-full", aria_label: "Help",
                                Info {}
                            }
                            TooltipContent { "Must be 3–20 characters, letters and numbers only." }
                        }
                    }
                }
                InputGroup {
                    InputGroupAddon { align: InputGroupAddonAlign::BlockStart, class: "border-b",
                        span { class: "text-sm font-medium text-foreground", "Username" }
                        Tooltip {
                            InputGroupButton {
                                size: InputGroupButtonSize::IconXs,
                                class: "ml-auto rounded-full",
                                aria_label: "Help",
                                Info {}
                            }
                            TooltipContent { "Must be 3–20 characters, letters and numbers only." }
                        }
                    }
                    InputGroupInput { id: "username-tooltip", placeholder: "john_doe" }
                }
            }
        }
    }
}
