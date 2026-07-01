---
title: "Demo Shimmer Marker"
name: "demo_shimmer_marker"
cargo_dependencies: []
registry_dependencies: ["marker", "spinner"]
type: "components:demos"
path: "demos/demo_shimmer_marker.rs"
---

# Demo Shimmer Marker

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_shimmer_marker
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::components::ui::spinner::Spinner;

#[component]
pub fn DemoShimmerMarker() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-4">
            <Marker role="status">
                <MarkerIcon><Spinner /></MarkerIcon>
                <MarkerContent class="shimmer">"Thinking..."</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator role="status">
                <MarkerContent class="shimmer">"Reading 4 files"</MarkerContent>
            </Marker>
        </div>
    }
}
```
