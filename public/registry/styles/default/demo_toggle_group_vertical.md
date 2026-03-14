---
title: "Demo Toggle Group Vertical"
name: "demo_toggle_group_vertical"
cargo_dependencies: []
registry_dependencies: ["toggle_group"]
type: "components:demos"
path: "demos/demo_toggle_group_vertical.rs"
---

# Demo Toggle Group Vertical

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_toggle_group_vertical
```

## Component Code

```rust
use icons::{AlignCenter, AlignLeft, AlignRight};
use leptos::prelude::*;

use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupOrientation};

#[component]
pub fn DemoToggleGroupVertical() -> impl IntoView {
    let align = RwSignal::new("left");

    view! {
        <ToggleGroup orientation=ToggleGroupOrientation::Vertical>
            <ToggleGroupItem
                title="Align Left"
                pressed=Memo::new(move |_| align.get() == "left")
                on:click=move |_| align.set("left")
            >
                <AlignLeft />
                "Left"
            </ToggleGroupItem>
            <ToggleGroupItem
                title="Align Center"
                pressed=Memo::new(move |_| align.get() == "center")
                on:click=move |_| align.set("center")
            >
                <AlignCenter />
                "Center"
            </ToggleGroupItem>
            <ToggleGroupItem
                title="Align Right"
                pressed=Memo::new(move |_| align.get() == "right")
                on:click=move |_| align.set("right")
            >
                <AlignRight />
                "Right"
            </ToggleGroupItem>
        </ToggleGroup>
    }
}
```
