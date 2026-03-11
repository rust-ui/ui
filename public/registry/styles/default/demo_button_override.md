---
title: "Demo Button Override"
name: "demo_button_override"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_button_override.rs"
---

# Demo Button Override

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_override
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButtonOverride() -> impl IntoView {
    view! { <Button class="hover:bg-pink-500 bg-sky-500">Fancy Button</Button> }
}
```
