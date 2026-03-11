---
title: "Demo Select Native Group"
name: "demo_select_native_group"
cargo_dependencies: []
registry_dependencies: ["select_native"]
type: "components:demos"
path: "demos/demo_select_native_group.rs"
---

# Demo Select Native Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_select_native_group
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::select_native::{LabelNative, SelectNative};

#[component]
pub fn DemoSelectNativeGroup() -> impl IntoView {
    const TARGET_ID: &str = "TARGET_NATIVE_GROUP";

    view! {
        <div class="space-y-2 min-w-[300px]">
            <LabelNative r#for=TARGET_ID>"Select with option groups (native)"</LabelNative>

            <SelectNative id=TARGET_ID>
                <optgroup label="Frontend">
                    <option value="1">React</option>
                    <option value="2">Vue</option>
                    <option value="3">Angular</option>
                </optgroup>
                <optgroup label="Backend">
                    <option value="4">Node.js</option>
                    <option value="5">Python</option>
                    <option value="6">Java</option>
                </optgroup>
            </SelectNative>
        </div>
    }
}
```
