---
title: "Demo Button Sizes"
name: "demo_button_sizes"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_button_sizes.rs"
---

# Demo Button Sizes

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_sizes
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize};

#[component]
pub fn DemoButtonSizes() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Button size=ButtonSize::Sm>Small</Button>
            <Button>Default</Button>
            <Button size=ButtonSize::Lg>Large</Button>
        </div>
    }
}
```
