---
title: "Demo Progress"
name: "demo_progress"
cargo_dependencies: []
registry_dependencies: ["progress"]
type: "components:demos"
path: "demos/demo_progress.rs"
---

# Demo Progress

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_progress
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::progress::Progress;

#[component]
pub fn DemoProgress() -> impl IntoView {
    view! {
        <div class="w-full max-w-sm">
            <Progress value=60.0 />
        </div>
    }
}
```
