---
title: "Demo Pressable"
name: "demo_pressable"
cargo_dependencies: []
registry_dependencies: ["button", "card", "pressable"]
type: "components:demos"
path: "demos/demo_pressable.rs"
---

# Demo Pressable

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_pressable
```

## Component Code

```rust
use icons::Bell;
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::Card;
use crate::components::ui::pressable::Pressable;

#[component]
pub fn DemoPressable() -> impl IntoView {
    view! {
        <Pressable>
            <Card class="flex gap-3 items-center py-3 px-4">
                <div class="flex justify-center items-center rounded-full size-9 bg-primary/10">
                    <Bell class="size-4 text-primary" />
                </div>
                <div class="flex-1">
                    <p class="text-sm font-medium">"New message received"</p>
                    <p class="text-xs text-muted-foreground">"2 minutes ago"</p>
                </div>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "Mark as read"
                </Button>
            </Card>
        </Pressable>
    }
}
```
