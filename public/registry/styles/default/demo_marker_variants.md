---
title: "Demo Marker Variants"
name: "demo_marker_variants"
cargo_dependencies: []
registry_dependencies: ["marker"]
type: "components:demos"
path: "demos/demo_marker_variants.rs"
---

# Demo Marker Variants

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_marker_variants
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerVariants() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-8 py-12">
            <Marker>
                <MarkerContent>"A default marker for inline notes."</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"A separator marker"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Border>
                <MarkerContent>"A border marker for row boundaries."</MarkerContent>
            </Marker>
        </div>
    }
}
```
