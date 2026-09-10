use dioxus::prelude::*;
use icons::{ChevronDown, Ellipsis};

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize, InputGroupInput,
};

#[component]
pub fn DemoInputGroupDropdown() -> Element {
    rsx! {
        div { class: "grid gap-4 w-full max-w-sm",
            InputGroup {
                InputGroupInput { placeholder: "Enter file name" }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    DropdownMenu {
                        DropdownMenuTrigger { as_child: true,
                            InputGroupButton { size: InputGroupButtonSize::IconXs, aria_label: "More options",
                                Ellipsis {}
                            }
                        }
                        DropdownMenuContent {
                            DropdownMenuGroup {
                                DropdownMenuItem { "Settings" }
                                DropdownMenuItem { "Copy path" }
                                DropdownMenuItem { "Open location" }
                            }
                        }
                    }
                }
            }

            InputGroup { class: "[--radius:1rem]",
                InputGroupInput { placeholder: "Enter search query" }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    DropdownMenu {
                        DropdownMenuTrigger { as_child: true,
                            InputGroupButton { size: InputGroupButtonSize::Xs, class: "gap-1 text-xs pr-2!",
                                "Search in..."
                                ChevronDown { class: "size-3" }
                            }
                        }
                        DropdownMenuContent {
                            DropdownMenuGroup {
                                DropdownMenuItem { "Documentation" }
                                DropdownMenuItem { "Blog Posts" }
                                DropdownMenuItem { "Changelog" }
                            }
                        }
                    }
                }
            }
        }
    }
}
