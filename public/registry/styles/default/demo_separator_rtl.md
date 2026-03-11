---
title: "Demo Separator Rtl"
name: "demo_separator_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "separator"]
type: "components:demos"
path: "demos/demo_separator_rtl.rs"
---

# Demo Separator Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_separator_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::separator::{Separator, SeparatorOrientation};

#[component]
pub fn DemoSeparatorRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-4">
                <h3 class="text-2xl font-bold text-pretty">"Rust/UI"</h3>
                <p>"مكتبة مكونات واجهة مستخدم مفتوحة المصدر 🦀."</p>

                <Separator />

                <div class="flex gap-4 items-center h-5">
                    <p>"المدونة"</p>
                    <Separator orientation=SeparatorOrientation::Vertical />
                    <p>"التوثيق"</p>
                    <Separator orientation=SeparatorOrientation::Vertical />
                    <p>"المصدر"</p>
                </div>
            </div>
        </DirectionProvider>
    }
}
```
