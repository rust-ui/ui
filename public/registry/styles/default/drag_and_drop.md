---
title: "Drag And Drop"
name: "drag_and_drop"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/drag_and_drop.rs"
---

# Drag And Drop

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add drag_and_drop
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Draggable, div, "flex flex-col gap-4 w-full max-w-4xl"}
    clx! {DraggableZone, div, "dragabble__container", "bg-neutral-600 p-4 mt-4"}

    // TODO. ItemRoot (needs `draggable` as clx attribute).
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DraggableItem(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <div class="p-4 border cursor-move border-input bg-card draggable" draggable="true">
            {text}
        </div>
    }
}
```
