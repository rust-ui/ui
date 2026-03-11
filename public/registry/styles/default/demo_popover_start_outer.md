---
title: "Demo Popover Start Outer"
name: "demo_popover_start_outer"
cargo_dependencies: []
registry_dependencies: ["popover"]
type: "components:demos"
path: "demos/demo_popover_start_outer.rs"
---

# Demo Popover Start Outer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_popover_start_outer
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverStartOuter() -> impl IntoView {
    view! {
        <Popover align=PopoverAlign::StartOuter>
            <PopoverTrigger>Popover (StartOuter)</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover StartOuter"</PopoverTitle>

                <PopoverDescription>
                    "StartOuter-aligned popover that positions completely to the left of the trigger button. The popover's right edge starts where the button's left edge ends."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
```
