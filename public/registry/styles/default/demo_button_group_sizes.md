---
title: "Demo Button Group Sizes"
name: "demo_button_group_sizes"
cargo_dependencies: []
registry_dependencies: ["button", "button_group"]
type: "components:demos"
path: "demos/demo_button_group_sizes.rs"
---

# Demo Button Group Sizes

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_group_sizes
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;

#[component]
pub fn DemoButtonGroupSizes() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <ButtonGroup>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "Cut"
                </Button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "Copy"
                </Button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "Paste"
                </Button>
            </ButtonGroup>

            <ButtonGroup>
                <Button variant=ButtonVariant::Outline>"Cut"</Button>
                <Button variant=ButtonVariant::Outline>"Copy"</Button>
                <Button variant=ButtonVariant::Outline>"Paste"</Button>
            </ButtonGroup>

            <ButtonGroup>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                    "Cut"
                </Button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                    "Copy"
                </Button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                    "Paste"
                </Button>
            </ButtonGroup>
        </div>
    }
}
```
