use dioxus::prelude::*;
use icons::{Pencil, Share2, Trash2};

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuActionVariant, DropdownMenuContent, DropdownMenuGroup,
    DropdownMenuItem, DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuDestructive() -> Element {
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { "Actions" }

            DropdownMenuContent {
                DropdownMenuGroup {
                    DropdownMenuItem {
                        DropdownMenuAction {
                            Pencil {}
                            "Edit"
                        }
                    }
                    DropdownMenuItem {
                        DropdownMenuAction {
                            Share2 {}
                            "Share"
                        }
                    }
                }

                Separator { class: "my-1" }

                DropdownMenuGroup {
                    DropdownMenuItem {
                        DropdownMenuAction { variant: DropdownMenuActionVariant::Destructive,
                            Trash2 {}
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
