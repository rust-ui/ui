---
title: "Demo Textarea Rtl"
name: "demo_textarea_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "textarea"]
type: "components:demos"
path: "demos/demo_textarea_rtl.rs"
---

# Demo Textarea Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_textarea_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::textarea::Textarea;

#[component]
pub fn DemoTextareaRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div>
                <Textarea attr:placeholder="اكتب رسالتك هنا." />
            </div>
        </DirectionProvider>
    }
}
```
