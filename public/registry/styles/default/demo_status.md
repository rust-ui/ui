---
title: "Demo Status"
name: "demo_status"
cargo_dependencies: []
registry_dependencies: ["status"]
type: "components:demos"
path: "demos/demo_status.rs"
---

# Demo Status

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_status
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::status::Status;

#[component]
pub fn DemoStatus() -> impl IntoView {
    view! {
        <Status>
            <div class="rounded-md size-16 bg-neutral-500" />
        </Status>
    }
}
```
