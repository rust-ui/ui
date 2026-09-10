use dioxus::prelude::*;

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem, DropdownMenuSubTrigger,
    DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenu() -> Element {
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { "Open Menu" }

            DropdownMenuContent {
                DropdownMenuLabel { "Main Menu" }

                DropdownMenuGroup {
                    DropdownMenuItem {
                        DropdownMenuAction { "Simple Item" }
                    }

                    DropdownMenuSub {
                        DropdownMenuSubTrigger { "Settings" }
                        DropdownMenuSubContent {
                            DropdownMenuSubItem { "Account Settings" }
                            DropdownMenuSubItem { "Privacy Settings" }
                            DropdownMenuSubItem { "Notification Settings" }
                        }
                    }

                    DropdownMenuSub {
                        DropdownMenuSubTrigger { "Tools" }
                        DropdownMenuSubContent {
                            DropdownMenuSubItem { "Export Data" }
                            DropdownMenuSubItem { "Import Data" }
                            Separator { class: "my-1" }
                            DropdownMenuSubItem { "Developer Tools" }
                        }
                    }
                }

                Separator { class: "my-1" }

                DropdownMenuGroup {
                    DropdownMenuItem {
                        DropdownMenuLink { href: "/", "Home" }
                    }
                    DropdownMenuItem {
                        DropdownMenuAction { "Sign Out" }
                    }
                }
            }
        }
    }
}
