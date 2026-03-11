---
title: "Demo Skeleton Text"
name: "demo_skeleton_text"
cargo_dependencies: []
registry_dependencies: ["skeleton"]
type: "components:demos"
path: "demos/demo_skeleton_text.rs"
---

# Demo Skeleton Text

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_skeleton_text
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonText() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 max-w-sm">
            <Skeleton class="w-full h-4" />
            <Skeleton class="h-4 w-[90%]" />
            <Skeleton class="w-full h-4" />
            <Skeleton class="h-4 w-[75%]" />
            <Skeleton class="h-4 w-[85%]" />
            <Skeleton class="h-4 w-[50%]" />
        </div>
    }
}
```
