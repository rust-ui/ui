---
title: "Demo Popover Rtl"
name: "demo_popover_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "popover"]
type: "components:demos"
path: "demos/demo_popover_rtl.rs"
---

# Demo Popover Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_popover_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::popover::{Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Popover>
                <PopoverTrigger>"فتح النافذة المنبثقة"</PopoverTrigger>

                <PopoverContent class="w-[300px]">
                    <PopoverTitle>"نافذة منبثقة تجريبية"</PopoverTitle>

                    <PopoverDescription>
                        "نافذة منبثقة تفاعلية تتكيف موضعها أثناء التمرير. جرّب التمرير لرؤية التمركز الذكي."
                    </PopoverDescription>
                </PopoverContent>
            </Popover>
        </DirectionProvider>
    }
}
```
