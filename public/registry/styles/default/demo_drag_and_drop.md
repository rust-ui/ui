---
title: "Demo Drag And Drop"
name: "demo_drag_and_drop"
cargo_dependencies: []
registry_dependencies: ["drag_and_drop"]
type: "components:demos"
path: "demos/demo_drag_and_drop.rs"
---

# Demo Drag And Drop

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_drag_and_drop
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::drag_and_drop::{Draggable, DraggableItem, DraggableZone};

#[component]
pub fn DemoDragAndDrop() -> impl IntoView {
    view! {
        <style>
            {".draggable.dragging {
            opacity: 0.5;
            }
            "}
        </style>

        <Draggable class="max-w-2xl">
            <DraggableZone>
                <DraggableItem text="1" />
                <DraggableItem text="2" />
            </DraggableZone>
            <DraggableZone>
                <DraggableItem text="3" />
                <DraggableItem text="4" />
            </DraggableZone>
        </Draggable>

        <script src="/components/drag_and_drop.js"></script>
    }
}
```
