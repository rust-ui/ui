---
title: "Demo Input Rtl"
name: "demo_input_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "input"]
type: "components:demos"
path: "demos/demo_input_rtl.rs"
---

# Demo Input Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::input::{Input, InputType};

#[component]
pub fn DemoInputRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="space-y-4 w-full max-w-sm">
                <Input placeholder="أدخل النص هنا" />
                <Input r#type=InputType::Email placeholder="البريد الإلكتروني" />
                <Input r#type=InputType::Password placeholder="كلمة المرور" />
            </div>
        </DirectionProvider>
    }
}
```
