---
title: "Demo Button Group Icon"
name: "demo_button_group_icon"
cargo_dependencies: []
registry_dependencies: ["button", "button_group"]
type: "components:demos"
path: "demos/demo_button_group_icon.rs"
---

# Demo Button Group Icon

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_group_icon
```

## Component Code

```rust
use icons::{Minus, Plus};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::button_group::{ButtonGroup, ButtonGroupOrientation};

#[component]
pub fn DemoButtonGroupIcon() -> impl IntoView {
    view! {
        <ButtonGroup orientation=ButtonGroupOrientation::Vertical>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Icon>
                <Plus />
            </Button>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Icon>
                <Minus />
            </Button>
        </ButtonGroup>
    }
}
```
