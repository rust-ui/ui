---
title: "Demo Input Group"
name: "demo_input_group"
cargo_dependencies: []
registry_dependencies: ["input_group"]
type: "components:demos"
path: "demos/demo_input_group.rs"
---

# Demo Input Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group
```

## Component Code

```rust
use icons::{Check, CreditCard, Info, Mail, Search, Star};
use leptos::prelude::*;

use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};

#[component]
pub fn DemoInputGroup() -> impl IntoView {
    view! {
        <div class="grid gap-6 w-full max-w-sm">
            <InputGroup>
                <InputGroupInput placeholder="Search..." />
                <InputGroupAddon>
                    <Search />
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Enter your email" />
                <InputGroupAddon>
                    <Mail />
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Card number" />
                <InputGroupAddon>
                    <CreditCard />
                </InputGroupAddon>
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <Check />
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Card number" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <Star />
                    <Info />
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
```
