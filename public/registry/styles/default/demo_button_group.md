---
title: "Demo Button Group"
name: "demo_button_group"
cargo_dependencies: []
registry_dependencies: ["button", "button_group"]
type: "components:demos"
path: "demos/demo_button_group.rs"
---

# Demo Button Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_group
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;

#[component]
pub fn DemoButtonGroup() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant=ButtonVariant::Outline>"First"</Button>
            <Button variant=ButtonVariant::Outline>"Second"</Button>
            <Button variant=ButtonVariant::Outline>"Third"</Button>
        </ButtonGroup>
    }
}
```
