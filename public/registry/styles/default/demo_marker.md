---
title: "Demo Marker"
name: "demo_marker"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["marker", "spinner"]
type: "components:demos"
path: "demos/demo_marker.rs"
---

# Demo Marker

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_marker
```

## Component Code

```rust
use icons::{GitBranch, Search};
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::components::ui::spinner::Spinner;

#[component]
pub fn DemoMarker() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-8 py-12">
            <Marker>
                <MarkerIcon><GitBranch /></MarkerIcon>
                <MarkerContent>"Switched to a new branch"</MarkerContent>
            </Marker>
            <Marker role="status">
                <MarkerIcon><Spinner /></MarkerIcon>
                <MarkerContent class="shimmer">"Thinking..."</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"Conversation compacted"</MarkerContent>
            </Marker>
            <Marker>
                <MarkerIcon><Search /></MarkerIcon>
                <MarkerContent>"Explored 4 files"</MarkerContent>
            </Marker>
        </div>
    }
}
```
