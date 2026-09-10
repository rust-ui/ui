use dioxus::prelude::*;
use icons::{ArrowUp, AtSign, ChevronDown, Globe, Paperclip, WandSparkles};

use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize, InputGroupTextarea,
};

#[component]
pub fn DemoInputGroupBlock() -> Element {
    rsx! {
        div { class: "w-full max-w-lg",
            InputGroup {
                InputGroupAddon { align: InputGroupAddonAlign::BlockStart,
                    InputGroupButton { size: InputGroupButtonSize::Xs,
                        AtSign { class: "size-4" }
                        "Add context"
                    }
                    Badge { variant: BadgeVariant::Secondary, "Vision" }
                    Badge { variant: BadgeVariant::Secondary, "Research" }
                }
                InputGroupTextarea {}
                InputGroupAddon { align: InputGroupAddonAlign::BlockEnd,
                    InputGroupButton { size: InputGroupButtonSize::IconXs,
                        Paperclip { class: "size-4" }
                    }
                    InputGroupButton { size: InputGroupButtonSize::Xs,
                        WandSparkles { class: "size-4" }
                        "Auto"
                        ChevronDown { class: "size-4" }
                    }
                    InputGroupButton { size: InputGroupButtonSize::Xs,
                        Globe { class: "size-4" }
                        "Sources"
                    }
                    div { class: "ml-auto",
                        InputGroupButton {
                            size: InputGroupButtonSize::IconXs,
                            class: "bg-primary text-primary-foreground hover:bg-primary/90",
                            ArrowUp { class: "size-4" }
                        }
                    }
                }
            }
        }
    }
}
