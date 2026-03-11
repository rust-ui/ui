---
title: "Demo Empty Input Group"
name: "demo_empty_input_group"
cargo_dependencies: []
registry_dependencies: ["empty", "input_group", "kbd"]
type: "components:demos"
path: "demos/demo_empty_input_group.rs"
---

# Demo Empty Input Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_empty_input_group
```

## Component Code

```rust
use icons::Search;
use leptos::prelude::*;

use crate::components::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle};
use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};
use crate::components::ui::kbd::Kbd;

#[component]
pub fn DemoEmptyInputGroup() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyTitle>"404 — Not Found"</EmptyTitle>
                <EmptyDescription>
                    "The page you're looking for doesn't exist. Try searching for what you need below."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <InputGroup class="sm:w-3/4">
                    <InputGroupInput placeholder="Try searching for pages..." />
                    <InputGroupAddon>
                        <Search />
                    </InputGroupAddon>
                    <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                        <Kbd>"/"</Kbd>
                    </InputGroupAddon>
                </InputGroup>
                <EmptyDescription>
                    "Need help? "
                    <a href="#" class="underline transition-colors underline-offset-4 hover:text-foreground">
                        "Contact support"
                    </a>
                </EmptyDescription>
            </EmptyContent>
        </Empty>
    }
}
```
