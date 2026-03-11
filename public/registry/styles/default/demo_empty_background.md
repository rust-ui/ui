---
title: "Demo Empty Background"
name: "demo_empty_background"
cargo_dependencies: []
registry_dependencies: ["button", "empty"]
type: "components:demos"
path: "demos/demo_empty_background.rs"
---

# Demo Empty Background

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_empty_background
```

## Component Code

```rust
use icons::{Bell, RefreshCcw};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyBackground() -> impl IntoView {
    view! {
        <Empty class="bg-muted/30">
            <EmptyHeader>
                <EmptyMedia variant=EmptyMediaVariant::Icon>
                    <Bell />
                </EmptyMedia>
                <EmptyTitle>"No Notifications"</EmptyTitle>
                <EmptyDescription>"You're all caught up. New notifications will appear here."</EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    <RefreshCcw />
                    "Refresh"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
```
