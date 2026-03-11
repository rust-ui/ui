---
title: "Demo Textarea"
name: "demo_textarea"
cargo_dependencies: []
registry_dependencies: ["textarea"]
type: "components:demos"
path: "demos/demo_textarea.rs"
---

# Demo Textarea

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_textarea
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::textarea::Textarea;

#[component]
pub fn DemoTextarea() -> impl IntoView {
    view! {
        <div>
            <Textarea attr:placeholder="Type your message here." />
        </div>
    }
}
```
