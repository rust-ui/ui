---
title: "Demo Skeleton Rtl"
name: "demo_skeleton_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "skeleton"]
type: "components:demos"
path: "demos/demo_skeleton_rtl.rs"
---

# Demo Skeleton Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_skeleton_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <div class="flex flex-col space-y-3">
                <Skeleton class="rounded-xl h-[125px] w-[250px]" />
                <div class="space-y-2">
                    <Skeleton class="h-4 w-[250px]" />
                    <Skeleton class="h-4 w-[200px]" />
                </div>
            </div>
        </DirectionProvider>
    }
}
```
