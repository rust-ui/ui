---
title: "Demo Popover End Outer"
name: "demo_popover_end_outer"
cargo_dependencies: []
registry_dependencies: ["popover"]
type: "components:demos"
path: "demos/demo_popover_end_outer.rs"
---

# Demo Popover End Outer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_popover_end_outer
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverEndOuter() -> impl IntoView {
    view! {
        <Popover align=PopoverAlign::EndOuter>
            <PopoverTrigger>Popover (EndOuter)</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover EndOuter"</PopoverTitle>

                <PopoverDescription>
                    "EndOuter-aligned popover that positions completely to the right of the trigger button. The popover's left edge starts where the button's right edge ends."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
```
