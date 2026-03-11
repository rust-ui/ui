---
title: "Demo Switch"
name: "demo_switch"
cargo_dependencies: []
registry_dependencies: ["switch"]
type: "components:demos"
path: "demos/demo_switch.rs"
---

# Demo Switch

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_switch
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::switch::{Switch, SwitchLabel};

#[component]
pub fn DemoSwitch() -> impl IntoView {
    view! {
        <div class="flex gap-2">
            <Switch />
            <SwitchLabel>"Airplane"</SwitchLabel>
        </div>
    }
}
```
