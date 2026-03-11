---
title: "Demo Label"
name: "demo_label"
cargo_dependencies: []
registry_dependencies: ["label"]
type: "components:demos"
path: "demos/demo_label.rs"
---

# Demo Label

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_label
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::label::Label;

#[component]
pub fn DemoLabel() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h2 class="text-2xl font-bold">Label Demo</h2>
            <div class="flex items-center space-x-2">
                // <Checkbox id="terms" checked=false />
                <Label html_for="terms">Accept terms and conditions</Label>
            </div>
        </div>
    }
}
```
