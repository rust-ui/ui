---
title: "Demo Dropdown Menu User"
name: "demo_dropdown_menu_user"
cargo_dependencies: []
registry_dependencies: ["dropdown_menu", "separator"]
type: "components:demos"
path: "demos/demo_dropdown_menu_user.rs"
---

# Demo Dropdown Menu User

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dropdown_menu_user
```

## Component Code

```rust
use icons::ChevronsUpDown;
use leptos::portal::Portal;
use leptos::prelude::*;

use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuLink, DropdownMenuPosition, DropdownMenuSub, DropdownMenuSubContent,
    DropdownMenuSubItem, DropdownMenuSubTrigger, DropdownMenuTrigger,
};
use crate::components::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuUser() -> impl IntoView {
    view! {
        // Mobile (xs, sm): dropdown appears on top instead of right
        <style>
            "
            @media (max-width: 767px) {
                /* Main dropdown content: position above trigger, not to the right */
                .dropdown-user-menu-content {
                    left: 8px !important;
                    right: auto !important;
                    width: 180px !important;
                    bottom: 70px !important;
                    top: auto !important;
                    z-index: 9999 !important;
                }
            }
            "
        </style>

        <DropdownMenu align=DropdownMenuAlign::EndOuter>
            <DropdownMenuTrigger class="flex justify-between px-2 w-full h-12 bg-transparent border-0">
                <div class="flex gap-2 items-center">
                    <span data-name="avatar" class="flex overflow-hidden relative rounded-lg size-8 shrink-0">
                        <span
                            data-name="avatar-fallback"
                            class="flex justify-center items-center rounded-full bg-secondary size-full"
                        >
                            RS
                        </span>
                    </span>

                    <div class="grid flex-1 text-sm leading-tight text-left">
                        <span class="font-medium truncate">"rustify.rs"</span>
                        <span class="text-xs truncate">"hello@example.com"</span>
                    </div>
                </div>

                <ChevronsUpDown />
            </DropdownMenuTrigger>

            <Portal>
                <DropdownMenuContent class="w-[220px] dropdown-user-menu-content" position=DropdownMenuPosition::Top>
                    <DropdownMenuLabel>"Main Menu"</DropdownMenuLabel>

                    <DropdownMenuGroup>
                        <DropdownMenuItem>
                            <DropdownMenuAction>"Simple Item"</DropdownMenuAction>
                        </DropdownMenuItem>

                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger>"Settings"</DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuSubItem>"Account Settings"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"Privacy Settings"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"Notification Settings"</DropdownMenuSubItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>

                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger>"Tools"</DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuSubItem>"Export Data"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"Import Data"</DropdownMenuSubItem>
                                <Separator class="my-1" />
                                <DropdownMenuSubItem>"Developer Tools"</DropdownMenuSubItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>
                    </DropdownMenuGroup>

                    <Separator class="my-1" />

                    <DropdownMenuGroup>
                        <DropdownMenuItem>
                            <DropdownMenuLink attr:href="/">"Home"</DropdownMenuLink>
                        </DropdownMenuItem>
                        <DropdownMenuItem>
                            <DropdownMenuAction>"Sign Out"</DropdownMenuAction>
                        </DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </Portal>
        </DropdownMenu>
    }
}
```
