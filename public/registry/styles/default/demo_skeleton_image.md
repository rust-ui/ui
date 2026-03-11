---
title: "Demo Skeleton Image"
name: "demo_skeleton_image"
cargo_dependencies: []
registry_dependencies: ["skeleton"]
type: "components:demos"
path: "demos/demo_skeleton_image.rs"
---

# Demo Skeleton Image

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_skeleton_image
```

## Component Code

```rust
use icons::Image;
use leptos::prelude::*;

use crate::components::ui::skeleton::Skeleton;

// TODO UI.

#[component]
pub fn DemoSkeletonImage() -> impl IntoView {
    view! {
        <div class="m-4 space-y-8 w-full md:flex md:items-center md:space-y-0 md:space-x-8 rtl:space-x-reverse">

            // TODO UI. Skeleton should be able to receive children (or not).
            <div class="flex justify-center items-center w-full h-48 rounded-lg animate-pulse sm:w-96 bg-muted">
                <Image class="text-muted-foreground size-10" />
            </div>

            <div class="space-y-2 w-full">
                <Skeleton class="h-4" />
                <Skeleton class="h-4 w-[80%]" />
                <Skeleton class="h-4 w-[80%]" />
                <Skeleton class="h-4 w-[80%]" />
            </div>
            <span class="hidden">Loading...</span>
        </div>
    }
}
```
