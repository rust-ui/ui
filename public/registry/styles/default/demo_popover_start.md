---
title: "Demo Popover Start"
name: "demo_popover_start"
cargo_dependencies: []
registry_dependencies: ["popover"]
type: "components:demos"
path: "demos/demo_popover_start.rs"
---

# Demo Popover Start

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_popover_start
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverStart() -> impl IntoView {
    view! {
        <Popover align=PopoverAlign::Start>
            <PopoverTrigger>Popover (Start)</PopoverTrigger>

            <PopoverContent class="w-[300px]">
                <PopoverTitle>"Popover Start"</PopoverTitle>

                <PopoverDescription>
                    "Start-aligned popover that anchors to the left edge. Watch how it repositions intelligently as you scroll."
                </PopoverDescription>
            </PopoverContent>
        </Popover>
    }
}
```
