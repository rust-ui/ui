---
title: "Demo Scroll Area Horizontal"
name: "demo_scroll_area_horizontal"
cargo_dependencies: []
registry_dependencies: ["scroll_area"]
type: "components:demos"
path: "demos/demo_scroll_area_horizontal.rs"
---

# Demo Scroll Area Horizontal

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_scroll_area_horizontal
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::scroll_area::ScrollArea;

#[component]
pub fn DemoScrollAreaHorizontal() -> impl IntoView {
    let images = (1..=5).map(|i| format!("Image {}", i)).collect::<Vec<_>>();

    view! {
        <ScrollArea class="w-96 whitespace-nowrap rounded-md border">
            <div class="flex gap-4 p-4 w-max">
                {images
                    .into_iter()
                    .map(|label| {
                        view! {
                            <div class="shrink-0">
                                <div class="overflow-hidden rounded-md">
                                    <div class="flex justify-center items-center text-sm h-[200px] w-[150px] bg-muted text-muted-foreground">
                                        {label}
                                    </div>
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </ScrollArea>
    }
}
```
