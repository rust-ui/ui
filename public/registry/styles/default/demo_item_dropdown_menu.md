---
title: "Demo Item Dropdown Menu"
name: "demo_item_dropdown_menu"
cargo_dependencies: []
registry_dependencies: ["avatar", "dropdown_menu", "item"]
type: "components:demos"
path: "demos/demo_item_dropdown_menu.rs"
---

# Demo Item Dropdown Menu

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_item_dropdown_menu
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::ui::item::{Item, ItemContent, ItemDescription, ItemMedia, ItemSize, ItemTitle};

#[component]
pub fn DemoItemDropdownMenu() -> impl IntoView {
    view! {
        <DropdownMenu align=DropdownMenuAlign::End>
            <DropdownMenuTrigger class="w-fit">"Dropdown"</DropdownMenuTrigger>

            <DropdownMenuContent class="w-72 [--radius:0.65rem]">
                {PEOPLE
                    .iter()
                    .map(|person| {
                        view! {
                            <DropdownMenuItem class="p-0">
                                <Item size=ItemSize::Sm class="p-2 w-full">
                                    <ItemMedia>
                                        <Avatar class="size-8">
                                            <AvatarFallback>{person.initials}</AvatarFallback>
                                        </Avatar>
                                    </ItemMedia>
                                    <ItemContent class="gap-0.5">
                                        <ItemTitle>{person.username}</ItemTitle>
                                        <ItemDescription>{person.email}</ItemDescription>
                                    </ItemContent>
                                </Item>
                            </DropdownMenuItem>
                        }
                    })
                    .collect::<Vec<_>>()}
            </DropdownMenuContent>
        </DropdownMenu>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

#[derive(Clone)]
struct Person {
    username: &'static str,
    initials: &'static str,
    email: &'static str,
}

const PEOPLE: &[Person] = &[
    Person { username: "Ryan Smith", initials: "RS", email: "ryan.smith@example.com" },
    Person { username: "Morgan Williams", initials: "MW", email: "morgan.williams@example.com" },
    Person { username: "Max Murphy", initials: "MM", email: "max.murphy@example.com" },
];
```
