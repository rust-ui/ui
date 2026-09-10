use dioxus::prelude::*;

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem, DropdownMenuSubTrigger,
    DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuUserIcon() -> Element {
    rsx! {
        DropdownMenu { align: DropdownMenuAlign::EndOuter,
            DropdownMenuTrigger { class: "p-0 bg-transparent border-0",
                div { class: "flex gap-2 items-center",
                    span { "data-name": "avatar", class: "flex overflow-hidden relative rounded-lg size-8 shrink-0",
                        span { "data-name": "avatar-fallback", class: "flex justify-center items-center rounded-lg bg-secondary size-full", "RS" }
                    }
                }
            }

            DropdownMenuContent { class: "w-[220px]",
                DropdownMenuGroup {
                    DropdownMenuItem { DropdownMenuAction { "Simple Item" } }
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
                    DropdownMenuItem { DropdownMenuLink { href: "/", "Home" } }
                    DropdownMenuItem { DropdownMenuAction { "Sign Out" } }
                }
            }
        }
    }
}
