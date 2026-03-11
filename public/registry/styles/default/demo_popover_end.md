---
title: "Demo Popover End"
name: "demo_popover_end"
cargo_dependencies: []
registry_dependencies: ["popover"]
type: "components:demos"
path: "demos/demo_popover_end.rs"
---

# Demo Popover End

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_popover_end
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverEnd() -> impl IntoView {
    view! {
        <Popover align=PopoverAlign::End>
            <PopoverTrigger>Popover (End)</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover End"</PopoverTitle>

                <PopoverDescription>
                    "End-aligned popover that anchors to the right edge. Notice the smart positioning behavior during scroll."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
```
