---
title: "Demo Button Group Input"
name: "demo_button_group_input"
cargo_dependencies: []
registry_dependencies: ["button", "button_group", "input"]
type: "components:demos"
path: "demos/demo_button_group_input.rs"
---

# Demo Button Group Input

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_group_input
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::button_group::ButtonGroup;
use crate::components::ui::input::Input;

#[component]
pub fn DemoButtonGroupInput() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Input placeholder="Enter a URL" class="w-64" />
            <Button>"Subscribe"</Button>
        </ButtonGroup>
    }
}
```
