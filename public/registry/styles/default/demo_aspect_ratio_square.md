---
title: "Demo Aspect Ratio Square"
name: "demo_aspect_ratio_square"
cargo_dependencies: []
registry_dependencies: ["aspect_ratio"]
type: "components:demos"
path: "demos/demo_aspect_ratio_square.rs"
---

# Demo Aspect Ratio Square

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_aspect_ratio_square
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::aspect_ratio::AspectRatio;

#[component]
pub fn DemoAspectRatioSquare() -> impl IntoView {
    view! {
        <div class="w-full max-w-[12rem]">
            <AspectRatio ratio=1.0 class="rounded-lg bg-muted">
                <div class="bg-gradient-to-br from-emerald-400 to-teal-600 rounded-lg size-full" />
            </AspectRatio>
        </div>
    }
}
```
