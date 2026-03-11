---
title: "Demo Combobox"
name: "demo_combobox"
cargo_dependencies: []
registry_dependencies: ["command", "popover"]
type: "components:demos"
path: "demos/demo_combobox.rs"
---

# Demo Combobox

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_combobox
```

## Component Code

```rust
use icons::{ChevronsUpDown, Search};
use leptos::prelude::*;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::components::ui::command::{Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList};
use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverTrigger};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
enum Language {
    Rust,
    JavaScript,
    Ruby,
    Python,
}

#[component]
pub fn DemoCombobox() -> impl IntoView {
    let value_signal = RwSignal::new(None::<Language>);

    view! {
        <Popover align=PopoverAlign::Start>
            <PopoverTrigger class="justify-between w-[200px]">
                <span class="truncate">
                    {move || value_signal.get().map(|l| l.to_string()).unwrap_or_else(|| "Select language...".into())}
                </span>
                <ChevronsUpDown class="ml-auto opacity-50 size-4" />
            </PopoverTrigger>

            <PopoverContent class="p-0 w-[200px]">
                <Command>
                    <div class="flex gap-2 items-center px-2 border-b">
                        <Search class="size-4 text-muted-foreground shrink-0" />
                        <CommandInput attr:placeholder="Search language..." />
                    </div>
                    <CommandList class="min-h-0">
                        <CommandEmpty>"No language found."</CommandEmpty>
                        <CommandGroup>
                            {Language::iter()
                                .map(|language| {
                                    let label = language.to_string();
                                    let is_selected = Signal::derive(move || value_signal.get() == Some(language));
                                    view! {
                                        <CommandItem
                                            value=label.clone()
                                            selected=is_selected
                                            on_select=Callback::new(move |_| {
                                                value_signal.set(Some(language));
                                            })
                                        >
                                            {label}
                                        </CommandItem>
                                    }
                                })
                                .collect_view()}
                        </CommandGroup>
                    </CommandList>
                </Command>
            </PopoverContent>
        </Popover>
    }
}
```
