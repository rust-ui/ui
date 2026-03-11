---
title: "Demo Separator"
name: "demo_separator"
cargo_dependencies: []
registry_dependencies: ["separator"]
type: "components:demos"
path: "demos/demo_separator.rs"
---

# Demo Separator

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_separator
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::separator::{Separator, SeparatorOrientation};

#[component]
pub fn DemoSeparator() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <h3 class="text-2xl font-bold text-pretty">"Rust/UI"</h3>
            <p>"An open-source UI component library 🦀."</p>

            <Separator />

            <div class="flex gap-4 items-center h-5">
                <p>"Blog"</p>
                <Separator orientation=SeparatorOrientation::Vertical />
                <p>"Docs"</p>
                <Separator orientation=SeparatorOrientation::Vertical />
                <p>"Source"</p>
            </div>
        </div>
    }
}
```
