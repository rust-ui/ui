---
title: "Demo Marker Shimmer"
name: "demo_marker_shimmer"
cargo_dependencies: []
registry_dependencies: ["marker"]
type: "components:demos"
path: "demos/demo_marker_shimmer.rs"
---

# Demo Marker Shimmer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_marker_shimmer
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerShimmer() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-8 py-12">
            <Marker role="status">
                <MarkerContent class="shimmer">"Thinking..."</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator role="status">
                <MarkerContent class="shimmer">"Reading 4 files"</MarkerContent>
            </Marker>
        </div>
    }
}
```
