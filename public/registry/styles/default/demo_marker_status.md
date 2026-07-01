---
title: "Demo Marker Status"
name: "demo_marker_status"
cargo_dependencies: []
registry_dependencies: ["marker", "spinner"]
type: "components:demos"
path: "demos/demo_marker_status.rs"
---

# Demo Marker Status

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_marker_status
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::components::ui::spinner::Spinner;

#[component]
pub fn DemoMarkerStatus() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Marker role="status">
                <MarkerIcon>
                    <Spinner />
                </MarkerIcon>
                <MarkerContent>"Compacting conversation"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator role="status">
                <MarkerIcon>
                    <Spinner />
                </MarkerIcon>
                <MarkerContent>"Running tests"</MarkerContent>
            </Marker>
        </div>
    }
}
```
