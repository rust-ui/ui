---
title: "Demo Dropdown Menu Select"
name: "demo_dropdown_menu_select"
cargo_dependencies: []
registry_dependencies: ["dropdown_menu"]
type: "components:demos"
path: "demos/demo_dropdown_menu_select.rs"
---

# Demo Dropdown Menu Select

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dropdown_menu_select
```

## Component Code

```rust
use icons::{ChevronsUpDown, LayoutTemplate, Sparkles, Star};
use leptos::prelude::*;

use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuRadioGroup, DropdownMenuRadioItem,
    DropdownMenuTrigger,
};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Registry {
    #[default]
    Components,
    Icons,
    Extensions,
}

impl Registry {
    fn label(self) -> &'static str {
        match self {
            Registry::Components => "Components",
            Registry::Icons => "Icons",
            Registry::Extensions => "Extensions",
        }
    }
}

#[component]
pub fn DemoDropdownMenuSelect() -> impl IntoView {
    let registry_signal = RwSignal::new(Registry::default());

    view! {
        <DropdownMenu align=DropdownMenuAlign::Center>
            <DropdownMenuTrigger class="flex justify-between px-2 w-full h-12 bg-transparent border-0">
                <div class="flex gap-2 items-center">
                    <div class="flex justify-center items-center rounded-lg bg-primary text-primary-foreground aspect-square size-8">
                        {move || match registry_signal.get() {
                            Registry::Components => view! { <LayoutTemplate /> }.into_any(),
                            Registry::Icons => view! { <Star /> }.into_any(),
                            Registry::Extensions => view! { <Sparkles /> }.into_any(),
                        }}
                    </div>

                    <div class="grid flex-1 text-sm leading-tight text-left">
                        <span class="font-medium">"Registry"</span>
                        <span class="text-xs">{move || registry_signal.get().label()}</span>
                    </div>
                </div>

                <ChevronsUpDown />
            </DropdownMenuTrigger>

            <DropdownMenuContent>
                <DropdownMenuRadioGroup value=registry_signal>
                    <DropdownMenuRadioItem value=Registry::Components>
                        <LayoutTemplate class="text-muted-foreground" />
                        "Components"
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value=Registry::Icons>
                        <Star class="text-muted-foreground" />
                        "Icons"
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value=Registry::Extensions>
                        <Sparkles class="text-muted-foreground" />
                        "Extensions"
                    </DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
```
