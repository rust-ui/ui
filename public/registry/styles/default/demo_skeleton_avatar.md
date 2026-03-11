---
title: "Demo Skeleton Avatar"
name: "demo_skeleton_avatar"
cargo_dependencies: []
registry_dependencies: ["skeleton"]
type: "components:demos"
path: "demos/demo_skeleton_avatar.rs"
---

# Demo Skeleton Avatar

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_skeleton_avatar
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::skeleton::Skeleton;

#[component]
pub fn DemoSkeletonAvatar() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <div class="flex gap-3 items-center">
                <Skeleton class="rounded-full size-10" />
                <div class="flex flex-col gap-2">
                    <Skeleton class="w-32 h-4" />
                    <Skeleton class="w-24 h-3" />
                </div>
            </div>
            <div class="flex gap-3 items-center">
                <Skeleton class="rounded-full size-10" />
                <div class="flex flex-col gap-2">
                    <Skeleton class="w-40 h-4" />
                    <Skeleton class="w-28 h-3" />
                </div>
            </div>
            <div class="flex gap-3 items-center">
                <Skeleton class="rounded-full size-10" />
                <div class="flex flex-col gap-2">
                    <Skeleton class="w-36 h-4" />
                    <Skeleton class="w-20 h-3" />
                </div>
            </div>
        </div>
    }
}
```
