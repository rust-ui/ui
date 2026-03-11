---
title: "Demo Button Rtl"
name: "demo_button_rtl"
cargo_dependencies: []
registry_dependencies: ["button", "direction_provider"]
type: "components:demos"
path: "demos/demo_button_rtl.rs"
---

# Demo Button Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoButtonRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Button>"زر"</Button>
        </DirectionProvider>
    }
}
```
