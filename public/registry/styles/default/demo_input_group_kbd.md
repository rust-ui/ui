---
title: "Demo Input Group Kbd"
name: "demo_input_group_kbd"
cargo_dependencies: []
registry_dependencies: ["input_group", "kbd"]
type: "components:demos"
path: "demos/demo_input_group_kbd.rs"
---

# Demo Input Group Kbd

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group_kbd
```

## Component Code

```rust
use icons::Search;
use leptos::prelude::*;

use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};
use crate::components::ui::kbd::Kbd;

#[component]
pub fn DemoInputGroupKbd() -> impl IntoView {
    view! {
        <InputGroup class="max-w-sm">
            <InputGroupAddon>
                <Search />
            </InputGroupAddon>
            <InputGroupInput placeholder="Search..." />
            <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                <Kbd>"⌘K"</Kbd>
            </InputGroupAddon>
        </InputGroup>
    }
}
```
