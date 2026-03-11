---
title: "Demo Skeleton Table"
name: "demo_skeleton_table"
cargo_dependencies: []
registry_dependencies: ["skeleton"]
type: "components:demos"
path: "demos/demo_skeleton_table.rs"
---

# Demo Skeleton Table

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_skeleton_table
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonTable() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 w-full max-w-lg">
            // Header row
            <div class="flex gap-4 pb-2 border-b">
                <Skeleton class="w-24 h-4" />
                <Skeleton class="w-32 h-4" />
                <Skeleton class="w-20 h-4" />
                <Skeleton class="ml-auto w-28 h-4" />
            </div>
            // Data rows
            {(0..5)
                .map(|_| {
                    view! {
                        <div class="flex gap-4 py-1">
                            <Skeleton class="w-24 h-4" />
                            <Skeleton class="w-32 h-4" />
                            <Skeleton class="w-20 h-4" />
                            <Skeleton class="ml-auto w-16 h-4" />
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
```
