---
title: "Demo Button Disabled"
name: "demo_button_disabled"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_button_disabled.rs"
---

# Demo Button Disabled

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_disabled
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButtonDisabled() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Button>"Normal"</Button>
            <Button attr:disabled=true>"Disabled"</Button>
        </div>
    }
}
```
