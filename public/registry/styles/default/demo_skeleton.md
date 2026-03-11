---
title: "Demo Skeleton"
name: "demo_skeleton"
cargo_dependencies: []
registry_dependencies: ["skeleton"]
type: "components:demos"
path: "demos/demo_skeleton.rs"
---

# Demo Skeleton

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_skeleton
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeleton() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-3">
            <Skeleton class="rounded-xl h-[125px] w-[250px]" />
            <div class="space-y-2">
                <Skeleton class="h-4 w-[250px]" />
                <Skeleton class="h-4 w-[200px]" />
            </div>
        </div>
    }
}
```
