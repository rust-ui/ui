---
title: "Demo Card"
name: "demo_card"
cargo_dependencies: []
registry_dependencies: ["button", "card"]
type: "components:demos"
path: "demos/demo_card.rs"
---

# Demo Card

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_card
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};

#[component]
pub fn DemoCard() -> impl IntoView {
    view! {
        <Card class="max-w-lg lg:max-w-2xl">
            <CardHeader>
                <CardTitle>"Card Title"</CardTitle>
            </CardHeader>

            <CardContent>
                <CardDescription>
                    "Hello, this is a more detailed description of the card content. You can add more text here to provide additional information about the card's purpose, features, or any other relevant details that might interest the viewer."
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
