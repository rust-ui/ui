---
title: "Demo Input Group Text"
name: "demo_input_group_text"
cargo_dependencies: []
registry_dependencies: ["input_group"]
type: "components:demos"
path: "demos/demo_input_group_text.rs"
---

# Demo Input Group Text

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group_text
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput, InputGroupText};

#[component]
pub fn DemoInputGroupText() -> impl IntoView {
    view! {
        <div class="grid gap-6 w-full max-w-sm">
            <InputGroup>
                <InputGroupAddon>
                    <InputGroupText>$</InputGroupText>
                </InputGroupAddon>
                <InputGroupInput placeholder="0.00" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <InputGroupText>USD</InputGroupText>
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupAddon>
                    <InputGroupText>{"https://"}</InputGroupText>
                </InputGroupAddon>
                <InputGroupInput placeholder="example.com" class="!pl-0.5" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <InputGroupText>{".com"}</InputGroupText>
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Enter your username" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <InputGroupText>@company.com</InputGroupText>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
```
