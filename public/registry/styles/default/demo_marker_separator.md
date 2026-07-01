---
title: "Demo Marker Separator"
name: "demo_marker_separator"
cargo_dependencies: []
registry_dependencies: ["marker"]
type: "components:demos"
path: "demos/demo_marker_separator.rs"
---

# Demo Marker Separator

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_marker_separator
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerSeparator() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"Today"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"Worked for 42s"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"Conversation compacted"</MarkerContent>
            </Marker>
        </div>
    }
}
```
