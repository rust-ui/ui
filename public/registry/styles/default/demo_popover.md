---
title: "Demo Popover"
name: "demo_popover"
cargo_dependencies: []
registry_dependencies: ["popover"]
type: "components:demos"
path: "demos/demo_popover.rs"
---

# Demo Popover

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_popover
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::popover::{Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopover() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>Open Popover</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover Demo"</PopoverTitle>

                <PopoverDescription>
                    "Interactive popover that adapts its position as you scroll. Try scrolling to see the smart positioning in action."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
```
