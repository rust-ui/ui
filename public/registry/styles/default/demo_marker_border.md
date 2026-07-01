---
title: "Demo Marker Border"
name: "demo_marker_border"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["marker"]
type: "components:demos"
path: "demos/demo_marker_border.rs"
---

# Demo Marker Border

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_marker_border
```

## Component Code

```rust
use icons::{FileText, GitBranch, Search};
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

#[component]
pub fn DemoMarkerBorder() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 py-12 w-full max-w-sm">
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon>
                    <GitBranch />
                </MarkerIcon>
                <MarkerContent>"Switched to release-candidate"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon>
                    <Search />
                </MarkerIcon>
                <MarkerContent>"Reviewed 8 related files"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon>
                    <FileText />
                </MarkerIcon>
                <MarkerContent>"Opened implementation notes"</MarkerContent>
            </Marker>
        </div>
    }
}
```
