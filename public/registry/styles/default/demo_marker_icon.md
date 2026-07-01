---
title: "Demo Marker Icon"
name: "demo_marker_icon"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["marker"]
type: "components:demos"
path: "demos/demo_marker_icon.rs"
---

# Demo Marker Icon

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_marker_icon
```

## Component Code

```rust
use icons::{BookOpenCheck, GitBranch, Search};
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

#[component]
pub fn DemoMarkerIcon() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-12 py-12">
            <Marker>
                <MarkerIcon><GitBranch /></MarkerIcon>
                <MarkerContent>"Switched to a new branch"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerIcon><Search /></MarkerIcon>
                <MarkerContent>"Explored 4 files"</MarkerContent>
            </Marker>
            // flex-col stacks icon above content
            <Marker class="flex-col">
                <MarkerIcon><BookOpenCheck /></MarkerIcon>
                <MarkerContent>"Syncing completed"</MarkerContent>
            </Marker>
        </div>
    }
}
```
