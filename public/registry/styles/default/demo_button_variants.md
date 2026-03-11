---
title: "Demo Button Variants"
name: "demo_button_variants"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_button_variants.rs"
---

# Demo Button Variants

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_variants
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoButtonVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center md:flex-row">
            <Button>Default</Button>

            <Button variant=ButtonVariant::Secondary>Secondary</Button>
            <Button variant=ButtonVariant::Outline>Outline</Button>
            <Button variant=ButtonVariant::Ghost>Ghost</Button>
            <Button variant=ButtonVariant::Destructive>Destructive</Button>
        </div>
    }
}
```
