---
title: "Demo Aspect Ratio"
name: "demo_aspect_ratio"
cargo_dependencies: []
registry_dependencies: ["aspect_ratio"]
type: "components:demos"
path: "demos/demo_aspect_ratio.rs"
---

# Demo Aspect Ratio

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_aspect_ratio
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::aspect_ratio::AspectRatio;

#[component]
pub fn DemoAspectRatio() -> impl IntoView {
    view! {
        <div class="w-full max-w-sm">
            <AspectRatio ratio=16.0_f64 / 9.0 class="rounded-lg bg-muted">
                <div class="bg-gradient-to-br from-violet-400 to-indigo-600 rounded-lg size-full" />
            </AspectRatio>
        </div>
    }
}
```
