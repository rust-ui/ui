---
title: "Demo Bubble Alignment"
name: "demo_bubble_alignment"
cargo_dependencies: []
registry_dependencies: ["bubble"]
type: "components:demos"
path: "demos/demo_bubble_alignment.rs"
---

# Demo Bubble Alignment

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bubble_alignment
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};

#[component]
pub fn DemoBubbleAlignment() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-8 py-12">
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>
                    "This bubble is aligned to the start. This is the default alignment."
                </BubbleContent>
            </Bubble>
            <Bubble align=BubbleAlign::End>
                <BubbleContent>
                    "This bubble is aligned to the end. Use this for user messages."
                </BubbleContent>
            </Bubble>
        </div>
    }
}
```
