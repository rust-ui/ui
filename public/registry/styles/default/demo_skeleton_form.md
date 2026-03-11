---
title: "Demo Skeleton Form"
name: "demo_skeleton_form"
cargo_dependencies: []
registry_dependencies: ["skeleton"]
type: "components:demos"
path: "demos/demo_skeleton_form.rs"
---

# Demo Skeleton Form

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_skeleton_form
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonForm() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 max-w-sm">
            <div class="flex flex-col gap-2">
                <Skeleton class="w-20 h-4" />
                <Skeleton class="w-full h-9 rounded-md" />
            </div>
            <div class="flex flex-col gap-2">
                <Skeleton class="w-16 h-4" />
                <Skeleton class="w-full h-9 rounded-md" />
            </div>
            <div class="flex flex-col gap-2">
                <Skeleton class="w-24 h-4" />
                <Skeleton class="w-full h-9 rounded-md" />
            </div>
            <Skeleton class="w-full h-9 rounded-md" />
        </div>
    }
}
```
