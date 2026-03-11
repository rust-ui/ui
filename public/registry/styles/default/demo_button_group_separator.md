---
title: "Demo Button Group Separator"
name: "demo_button_group_separator"
cargo_dependencies: []
registry_dependencies: ["button", "button_group"]
type: "components:demos"
path: "demos/demo_button_group_separator.rs"
---

# Demo Button Group Separator

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_group_separator
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::button_group::{ButtonGroup, ButtonGroupSeparator};

#[component]
pub fn DemoButtonGroupSeparator() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant=ButtonVariant::Secondary>"Copy"</Button>
            <ButtonGroupSeparator />
            <Button variant=ButtonVariant::Secondary>"Paste"</Button>
            <ButtonGroupSeparator />
            <Button variant=ButtonVariant::Secondary>"Cut"</Button>
        </ButtonGroup>
    }
}
```
