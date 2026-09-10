use dioxus::prelude::*;
use icons::ChevronsUpDown;

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem,
    DropdownMenuSubTrigger, DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuUser() -> Element {
    rsx! {
        style { r#"
            @media (max-width: 767px) {{
                .dropdown-user-menu-content {{
                    left: 8px !important;
                    right: auto !important;
                    width: 180px !important;
                    bottom: 70px !important;
                    top: auto !important;
                    z-index: 9999 !important;
                }}
            }}
        "# }

        DropdownMenu { align: DropdownMenuAlign::EndOuter,
            DropdownMenuTrigger { class: "flex justify-between px-2 w-full h-12 bg-transparent border-0",
                div { class: "flex gap-2 items-center",
                    span { "data-name": "avatar", class: "flex overflow-hidden relative rounded-lg size-8 shrink-0",
                        span { "data-name": "avatar-fallback", class: "flex justify-center items-center rounded-full bg-secondary size-full", "RS" }
                    }
                    div { class: "grid flex-1 text-sm leading-tight text-left",
                        span { class: "font-medium truncate", "rustify.rs" }
                        span { class: "text-xs truncate", "hello@example.com" }
                    }
                }
                ChevronsUpDown {}
            }

            DropdownMenuContent { class: "w-[220px] dropdown-user-menu-content", position: crate::ui::dropdown_menu::DropdownMenuPosition::Top,
                DropdownMenuLabel { "Main Menu" }
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
