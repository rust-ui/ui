---
title: "Demo Input Group Spinner"
name: "demo_input_group_spinner"
cargo_dependencies: []
registry_dependencies: ["input_group", "spinner"]
type: "components:demos"
path: "demos/demo_input_group_spinner.rs"
---

# Demo Input Group Spinner

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group_spinner
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput, InputGroupText};
use crate::components::ui::spinner::{Spinner, SpinnerCircle};

#[component]
pub fn DemoInputGroupSpinner() -> impl IntoView {
    view! {
        <div class="grid gap-4 w-full max-w-sm">
            <InputGroup>
                <InputGroupInput placeholder="Searching..." />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupAddon>
                    <SpinnerCircle />
                </InputGroupAddon>
                <InputGroupInput placeholder="Processing..." />
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Saving changes..." />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <InputGroupText>"Saving..."</InputGroupText>
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
```
