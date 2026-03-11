---
title: "Demo Switch Rtl"
name: "demo_switch_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "switch"]
type: "components:demos"
path: "demos/demo_switch_rtl.rs"
---

# Demo Switch Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_switch_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::switch::{Switch, SwitchLabel};

#[component]
pub fn DemoSwitchRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <div class="flex gap-2">
                <Switch />
                <SwitchLabel>"وضع الطائرة"</SwitchLabel>
            </div>
        </DirectionProvider>
    }
}
```
