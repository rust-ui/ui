---
title: "Demo Empty"
name: "demo_empty"
cargo_dependencies: []
registry_dependencies: ["button", "empty"]
type: "components:demos"
path: "demos/demo_empty.rs"
---

# Demo Empty

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_empty
```

## Component Code

```rust
use icons::{ArrowUpRight, FolderCode};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmpty() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia variant=EmptyMediaVariant::Icon>
                    <FolderCode />
                </EmptyMedia>
                <EmptyTitle>"No Projects Yet"</EmptyTitle>
                <EmptyDescription>
                    "You haven't created any projects yet. Get started by creating your first project."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <div class="flex gap-2">
                    <Button>"Create Project"</Button>
                    <Button variant=ButtonVariant::Outline>"Import Project"</Button>
                </div>
            </EmptyContent>
            <Button variant=ButtonVariant::Link size=ButtonSize::Sm class="text-muted-foreground">
                <a href="#" class="flex gap-1 items-center">
                    <span>"Learn More"</span>
                    <ArrowUpRight />
                </a>
            </Button>
        </Empty>
    }
}
```
