---
title: "Demo Dropdown Menu Destructive"
name: "demo_dropdown_menu_destructive"
cargo_dependencies: []
registry_dependencies: ["dropdown_menu", "separator"]
type: "components:demos"
path: "demos/demo_dropdown_menu_destructive.rs"
---

# Demo Dropdown Menu Destructive

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dropdown_menu_destructive
```

## Component Code

```rust
use icons::{Pencil, Share2, Trash2};
use leptos::prelude::*;

use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuActionVariant, DropdownMenuContent, DropdownMenuGroup,
    DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuDestructive() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>"Actions"</DropdownMenuTrigger>

            <DropdownMenuContent>
                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuAction>
                            <Pencil />
                            "Edit"
                        </DropdownMenuAction>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        <DropdownMenuAction>
                            <Share2 />
                            "Share"
                        </DropdownMenuAction>
                    </DropdownMenuItem>
                </DropdownMenuGroup>

                <Separator class="my-1" />

                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuAction variant=DropdownMenuActionVariant::Destructive>
                            <Trash2 />
                            "Delete"
                        </DropdownMenuAction>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
```
