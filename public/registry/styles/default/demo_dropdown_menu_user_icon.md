---
title: "Demo Dropdown Menu User Icon"
name: "demo_dropdown_menu_user_icon"
cargo_dependencies: []
registry_dependencies: ["dropdown_menu", "separator"]
type: "components:demos"
path: "demos/demo_dropdown_menu_user_icon.rs"
---

# Demo Dropdown Menu User Icon

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dropdown_menu_user_icon
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem,
    DropdownMenuSubTrigger, DropdownMenuTrigger,
};
use crate::components::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuUserIcon() -> impl IntoView {
    view! {
        <DropdownMenu align=DropdownMenuAlign::EndOuter>
            <DropdownMenuTrigger class="p-0 bg-transparent border-0">
                <div class="flex gap-2 items-center">
                    <span data-name="avatar" class="flex overflow-hidden relative rounded-lg size-8 shrink-0">
                        <span
                            data-name="avatar-fallback"
                            class="flex justify-center items-center rounded-lg bg-secondary size-full"
                        >
                            RS
                        </span>
                    </span>
                </div>
            </DropdownMenuTrigger>

            <DropdownMenuContent class="w-[220px]">
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
        </DropdownMenu>
    }
}
```
