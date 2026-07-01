---
title: "Demo Bubble Group"
name: "demo_bubble_group"
cargo_dependencies: []
registry_dependencies: ["bubble"]
type: "components:demos"
path: "demos/demo_bubble_group.rs"
---

# Demo Bubble Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bubble_group
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};

#[component]
pub fn DemoBubbleGroup() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-8 py-12">
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>"Can you tell me what's the issue?"</BubbleContent>
            </Bubble>
            <BubbleGroup>
                <Bubble align=BubbleAlign::End>
                    <BubbleContent>"You tell me!"</BubbleContent>
                </Bubble>
                <Bubble align=BubbleAlign::End>
                    <BubbleContent>"It worked yesterday. You broke it!"</BubbleContent>
                </Bubble>
                <Bubble align=BubbleAlign::End>
                    <BubbleContent>"Find the bug and fix it."</BubbleContent>
                    <BubbleReactions
                        attr:aria-label="Reactions: eyes"
                        align=BubbleAlign::Start
                    >
                        <span>"👀"</span>
                    </BubbleReactions>
                </Bubble>
            </BubbleGroup>
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>
                    "Want me to diff yesterday's you against today's you? It's a bit embarrassing."
                </BubbleContent>
            </Bubble>
        </div>
    }
}
```
