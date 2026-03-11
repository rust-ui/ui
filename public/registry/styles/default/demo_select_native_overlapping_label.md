---
title: "Demo Select Native Overlapping Label"
name: "demo_select_native_overlapping_label"
cargo_dependencies: []
registry_dependencies: ["select_native"]
type: "components:demos"
path: "demos/demo_select_native_overlapping_label.rs"
---

# Demo Select Native Overlapping Label

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_select_native_overlapping_label
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::select_native::{OverlappingLabel, SelectNative};

#[component]
pub fn DemoSelectNativeOverlappingLabel() -> impl IntoView {
    const TARGET_ID: &str = "TARGET_NATIVE_OVERLAPPING_LABEL";

    view! {
        <div class="relative group min-w-[300px]">
            <OverlappingLabel r#for=TARGET_ID label="Select with overlapping label (native)" />

            <SelectNative id=TARGET_ID>
                <option value="" disabled selected>
                    Select framework
                </option>
                <option value="1">React</option>
                <option value="2">Next.js</option>
                <option value="3">Astro</option>
                <option value="4">Gatsby</option>
            </SelectNative>
        </div>
    }
}
```
