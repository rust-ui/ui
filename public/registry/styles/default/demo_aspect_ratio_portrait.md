---
title: "Demo Aspect Ratio Portrait"
name: "demo_aspect_ratio_portrait"
cargo_dependencies: []
registry_dependencies: ["aspect_ratio"]
type: "components:demos"
path: "demos/demo_aspect_ratio_portrait.rs"
---

# Demo Aspect Ratio Portrait

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_aspect_ratio_portrait
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::aspect_ratio::AspectRatio;

#[component]
pub fn DemoAspectRatioPortrait() -> impl IntoView {
    view! {
        <div class="w-full max-w-[10rem]">
            <AspectRatio ratio=9.0_f64 / 16.0 class="rounded-lg bg-muted">
                <div class="bg-gradient-to-br from-rose-400 to-pink-600 rounded-lg size-full" />
            </AspectRatio>
        </div>
    }
}
```
