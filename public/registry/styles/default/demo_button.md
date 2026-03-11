---
title: "Demo Button"
name: "demo_button"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_button.rs"
---

# Demo Button

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButton() -> impl IntoView {
    view! { <Button>"Button"</Button> }
}
```
