---
title: "Demo Card Sm"
name: "demo_card_sm"
cargo_dependencies: []
registry_dependencies: ["button", "card"]
type: "components:demos"
path: "demos/demo_card_sm.rs"
---

# Demo Card Sm

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_card_sm
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle};

#[component]
pub fn DemoCardSm() -> impl IntoView {
    view! {
        <Card size=CardSize::Sm class="max-w-sm">
            <CardHeader>
                <CardTitle>"Card Title"</CardTitle>
            </CardHeader>

            <CardContent>
                <CardDescription>
                    "A compact card with reduced padding, ideal for dense UI panels like customizers or sidebars."
                </CardDescription>
            </CardContent>

            <CardFooter class="justify-end">
                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                <Button>"Confirm"</Button>
            </CardFooter>
        </Card>
    }
}
```
