---
title: "Demo Scroll Area"
name: "demo_scroll_area"
cargo_dependencies: []
registry_dependencies: ["scroll_area", "separator"]
type: "components:demos"
path: "demos/demo_scroll_area.rs"
---

# Demo Scroll Area

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_scroll_area
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::scroll_area::ScrollArea;
use crate::components::ui::separator::Separator;

#[component]
pub fn DemoScrollArea() -> impl IntoView {
    let tags = (0..=50).rev().map(|i| format!("v1.2.0-beta.{}", i)).collect::<Vec<_>>();

    view! {
        <ScrollArea class="w-48 h-72 rounded-md border">
            <div class="p-4">
                <h4 class="mb-4 text-sm font-medium leading-none">Tags</h4>
                {tags
                    .into_iter()
                    .map(|tag| {
                        view! {
                            <>
                                <div class="text-sm">{tag}</div>
                                <Separator class="my-2" />
                            </>
                        }
                    })
                    .collect_view()}
            </div>
        </ScrollArea>
    }
}
```
