---
title: "Demo Select Native Auto Width"
name: "demo_select_native_auto_width"
cargo_dependencies: []
registry_dependencies: ["select_native"]
type: "components:demos"
path: "demos/demo_select_native_auto_width.rs"
---

# Demo Select Native Auto Width

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_select_native_auto_width
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::select_native::{LabelNative, SelectNative};

#[component]
pub fn DemoSelectNativeAutoWidth() -> impl IntoView {
    const TARGET_ID: &str = "TARGET_NATIVE_AUTO_WIDTH";

    view! {
        <div class="space-y-2 min-w-[300px]">
            <LabelNative r#for=TARGET_ID>"Select with auto-width (native)"</LabelNative>

            <div class="w-fit">
                <SelectNative id=TARGET_ID>
                    <option value="1">React</option>
                    <option value="2">Next.js</option>
                    <option value="3">Astro</option>
                    <option value="4">Gatsby</option>
                </SelectNative>
            </div>
        </div>
    }
}
```
