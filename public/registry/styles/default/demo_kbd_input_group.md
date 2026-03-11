---
title: "Demo Kbd Input Group"
name: "demo_kbd_input_group"
cargo_dependencies: []
registry_dependencies: ["input_group", "kbd"]
type: "components:demos"
path: "demos/demo_kbd_input_group.rs"
---

# Demo Kbd Input Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_kbd_input_group
```

## Component Code

```rust
use icons::Search;
use leptos::prelude::*;

use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};
use crate::components::ui::kbd::Kbd;

#[component]
pub fn DemoKbdInputGroup() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 w-full max-w-xs">
            <InputGroup>
                <InputGroupInput placeholder="Search..." />
                <InputGroupAddon>
                    <Search />
                </InputGroupAddon>
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <Kbd>"⌘"</Kbd>
                    <Kbd>"K"</Kbd>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
```
