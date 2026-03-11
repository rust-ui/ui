---
title: "Demo Card Action"
name: "demo_card_action"
cargo_dependencies: []
registry_dependencies: ["badge", "card"]
type: "components:demos"
path: "demos/demo_card_action.rs"
---

# Demo Card Action

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_card_action
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::card::{Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn DemoCardAction() -> impl IntoView {
    view! {
        <div class="grid gap-4 max-w-2xl sm:grid-cols-2">
            <Card>
                <CardHeader>
                    <CardDescription>"Total Revenue"</CardDescription>
                    <CardTitle class="text-2xl font-bold tabular-nums">"$45,231.89"</CardTitle>
                    <CardAction>
                        <Badge variant=BadgeVariant::Secondary>"+12.5%"</Badge>
                    </CardAction>
                </CardHeader>
                <CardContent>
                    <p class="text-sm text-muted-foreground">"+20.1% from last month"</p>
                </CardContent>
            </Card>

            <Card>
                <CardHeader>
                    <CardDescription>"Active Users"</CardDescription>
                    <CardTitle class="text-2xl font-bold tabular-nums">"2,350"</CardTitle>
                    <CardAction>
                        <Badge variant=BadgeVariant::Destructive>"-3.2%"</Badge>
                    </CardAction>
                </CardHeader>
                <CardContent>
                    <p class="text-sm text-muted-foreground">"-180 from last month"</p>
                </CardContent>
            </Card>
        </div>
    }
}
```
