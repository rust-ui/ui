---
title: "Demo Collapsible"
name: "demo_collapsible"
cargo_dependencies: []
registry_dependencies: ["collapsible"]
type: "components:demos"
path: "demos/demo_collapsible.rs"
---

# Demo Collapsible

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_collapsible
```

## Component Code

```rust
use icons::ChevronsUpDown;
use leptos::prelude::*;

use crate::components::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn DemoCollapsible() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Collapsible open=open class="flex flex-col gap-2 w-[350px]">
            <div class="flex gap-4 justify-between items-center px-4">
                <h4 class="text-sm font-semibold">"Order #4189"</h4>
                <CollapsibleTrigger class="inline-flex justify-center items-center rounded-md transition-colors size-8 hover:bg-accent hover:text-accent-foreground">
                    <ChevronsUpDown class="size-4" />
                    <span class="sr-only">"Toggle details"</span>
                </CollapsibleTrigger>
            </div>
            <div class="flex justify-between items-center py-2 px-4 text-sm rounded-md border">
                <span class="text-muted-foreground">"Status"</span>
                <span class="font-medium">"Shipped"</span>
            </div>
            <CollapsibleContent class="flex flex-col gap-2">
                <div class="py-2 px-4 text-sm rounded-md border">
                    <p class="font-medium">"Shipping address"</p>
                    <p class="text-muted-foreground">"100 Market St, San Francisco"</p>
                </div>
                <div class="py-2 px-4 text-sm rounded-md border">
                    <p class="font-medium">"Items"</p>
                    <p class="text-muted-foreground">"2x Studio Headphones"</p>
                </div>
            </CollapsibleContent>
        </Collapsible>
    }
}
```
