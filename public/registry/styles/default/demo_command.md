---
title: "Demo Command"
name: "demo_command"
cargo_dependencies: []
registry_dependencies: ["command", "input_group", "kbd"]
type: "components:demos"
path: "demos/demo_command.rs"
---

# Demo Command

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_command
```

## Component Code

```rust
use icons::{ArrowDown, ArrowRight, ArrowUp, CircleDashed, CornerDownLeft, Search};
use leptos::prelude::*;
use strum::Display;

use crate::components::ui::command::{
    Command, CommandDescription, CommandFooter, CommandGroup, CommandGroupLabel, CommandHeader, CommandInput,
    CommandItemLink, CommandList, CommandTitle,
};
use crate::components::ui::input_group::{InputGroup, InputGroupAddon};
use crate::components::ui::kbd::Kbd;

#[component]
pub fn DemoCommand() -> impl IntoView {
    view! {
        <div data-name="__DemoCard" class="my-6 mx-auto w-full rounded-md border max-w-[450px]">
            <CommandHeader>
                <CommandTitle>Search documentation...</CommandTitle>
                <CommandDescription>Search for a command to run...</CommandDescription>
            </CommandHeader>
            <Command>
                <InputGroup class="h-9 rounded-none border-b">
                    <InputGroupAddon>
                        <Search />
                    </InputGroupAddon>
                    <CommandInput
                        attr:placeholder="Search documentation..."
                        class="flex-1 py-0 h-9 rounded-none border-0 shadow-none"
                    />
                </InputGroup>
                <CommandList attr:id="command_demo" attr:tabindex="-1">
                    {[(CommandCategory::Pages, PAGES_ITEMS), (CommandCategory::Components, COMPONENTS_ITEMS)]
                        .into_iter()
                        .map(|(category, items)| {
                            view! {
                                <CommandGroup attr:role="presentation" class="p-0">
                                    <CommandGroupLabel attr:aria-hidden="true" class="p-3">
                                        {category.to_string()}
                                    </CommandGroupLabel>
                                    {items
                                        .iter()
                                        .map(|item| {
                                            let icon = item.icon();
                                            view! {
                                                <CommandItemLink
                                                    class="px-3"
                                                    attr:href=item.href
                                                    attr:target="_blank"
                                                    attr:rel="noopener noreferrer"
                                                >
                                                    {icon}
                                                    <span>{item.label}</span>
                                                </CommandItemLink>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </CommandGroup>
                            }
                        })
                        .collect::<Vec<_>>()}
                </CommandList>
            </Command>
            <CommandFooter>
                <div class="flex gap-2 items-center">
                    <Kbd>
                        <ArrowUp />
                    </Kbd>
                    <Kbd>
                        <ArrowDown />
                    </Kbd>
                    <span>Navigate</span>
                </div>
                <div class="flex gap-2 items-center">
                    <Kbd>
                        <CornerDownLeft />
                    </Kbd>
                    <span>Go to Page</span>
                </div>
            </CommandFooter>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ DATA STRUCTURES ✨                  */
/* ========================================================== */

#[derive(Clone, Debug, Display)]
enum CommandCategory {
    Pages,
    Components,
}

#[derive(Clone, Debug)]
struct CommandItemData {
    label: &'static str,
    href: &'static str,
    category: CommandCategory,
}

impl CommandItemData {
    fn icon(&self) -> AnyView {
        match self.category {
            CommandCategory::Pages => view! { <ArrowRight /> }.into_any(),
            CommandCategory::Components => view! { <CircleDashed /> }.into_any(),
        }
    }
}

const PAGES_ITEMS: &[CommandItemData] = &[
    CommandItemData { label: "Docs", href: "/docs", category: CommandCategory::Pages },
    CommandItemData { label: "Components", href: "/components", category: CommandCategory::Pages },
    CommandItemData { label: "Blocks", href: "/blocks", category: CommandCategory::Pages },
];

const COMPONENTS_ITEMS: &[CommandItemData] = &[
    CommandItemData { label: "Accordion", href: "/components/accordion", category: CommandCategory::Components },
    CommandItemData { label: "Alert", href: "/components/alert", category: CommandCategory::Components },
    CommandItemData { label: "Alert Dialog", href: "/components/alert-dialog", category: CommandCategory::Components },
    CommandItemData { label: "Avatar", href: "/components/avatar", category: CommandCategory::Components },
    CommandItemData { label: "Badge", href: "/components/badge", category: CommandCategory::Components },
    CommandItemData { label: "Breadcrumb", href: "/components/breadcrumb", category: CommandCategory::Components },
];
```
