---
title: "Demo Label Rtl"
name: "demo_label_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "input", "label"]
type: "components:demos"
path: "demos/demo_label_rtl.rs"
---

# Demo Label Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_label_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

#[component]
pub fn DemoLabelRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-2">
                <Label html_for="rtl-label-input">"اسم المستخدم"</Label>
                <Input attr:id="rtl-label-input" placeholder="أدخل اسم المستخدم" />
            </div>
        </DirectionProvider>
    }
}
```
