---
title: "Demo Dropdown Menu Radio"
name: "demo_dropdown_menu_radio"
cargo_dependencies: []
registry_dependencies: ["dropdown_menu"]
type: "components:demos"
path: "demos/demo_dropdown_menu_radio.rs"
---

# Demo Dropdown Menu Radio

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dropdown_menu_radio
```

## Component Code

```rust
use icons::{ArrowDownWideNarrow, ArrowUpNarrowWide, ChevronDown, CircleX};
use leptos::prelude::*;

use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuRadioGroup,
    DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuTrigger,
};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    None,
    Asc,
    Desc,
}

impl SortDirection {
    fn label(self) -> &'static str {
        match self {
            SortDirection::None => "No sort",
            SortDirection::Asc => "Ascending",
            SortDirection::Desc => "Descending",
        }
    }
}

#[component]
pub fn DemoDropdownMenuRadio() -> impl IntoView {
    let sort_signal = RwSignal::new(SortDirection::default());

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger class="flex gap-2 justify-between items-center">
                <span>"Sort: "</span>
                <span class="text-muted-foreground">{move || sort_signal.get().label()}</span>
                <ChevronDown class="size-4 text-muted-foreground" />
            </DropdownMenuTrigger>

            <DropdownMenuContent>
                <DropdownMenuRadioGroup value=sort_signal>
                    <DropdownMenuRadioItem value=SortDirection::Asc>
                        <ArrowUpNarrowWide class="text-muted-foreground" />
                        "Sort asc"
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value=SortDirection::Desc>
                        <ArrowDownWideNarrow class="text-muted-foreground" />
                        "Sort desc"
                    </DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>

                {move || {
                    (sort_signal.get() != SortDirection::None)
                        .then(|| {
                            view! {
                                <DropdownMenuItem on:click=move |_| sort_signal.set(SortDirection::None)>
                                    <DropdownMenuAction>
                                        <CircleX class="text-muted-foreground" />
                                        "Remove sort"
                                    </DropdownMenuAction>
                                </DropdownMenuItem>
                            }
                        })
                }}

                <DropdownMenuSeparator />

                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuAction>"Other action"</DropdownMenuAction>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
```
