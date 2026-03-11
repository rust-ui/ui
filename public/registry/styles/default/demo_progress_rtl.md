---
title: "Demo Progress Rtl"
name: "demo_progress_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "progress"]
type: "components:demos"
path: "demos/demo_progress_rtl.rs"
---

# Demo Progress Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_progress_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::progress::Progress;

#[component]
pub fn DemoProgressRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-4 w-full max-w-sm">
                <p class="text-sm text-muted-foreground">"تقدم التحميل"</p>
                <Progress value=60.0 />
            </div>
        </DirectionProvider>
    }
}
```
