---
title: "Demo Empty Card"
name: "demo_empty_card"
cargo_dependencies: []
registry_dependencies: ["button", "card", "empty"]
type: "components:demos"
path: "demos/demo_empty_card.rs"
---

# Demo Empty Card

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_empty_card
```

## Component Code

```rust
use icons::{ArrowUpRight, Folder};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent};
use crate::components::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyCard() -> impl IntoView {
    view! {
        <Card class="w-full max-w-md">
            <CardContent class="p-0">
                <Empty>
                    <EmptyHeader>
                        <EmptyMedia variant=EmptyMediaVariant::Icon>
                            <Folder />
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
                        <Button variant=ButtonVariant::Link class="text-muted-foreground">
                            <a href="#" class="flex gap-1 items-center">
                                <span>"Learn More"</span>
                                <ArrowUpRight />
                            </a>
                        </Button>
                    </EmptyContent>
                </Empty>
            </CardContent>
        </Card>
    }
}
```
