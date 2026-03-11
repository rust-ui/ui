---
title: "Demo Spinner Button"
name: "demo_spinner_button"
cargo_dependencies: []
registry_dependencies: ["button", "spinner"]
type: "components:demos"
path: "demos/demo_spinner_button.rs"
---

# Demo Spinner Button

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_spinner_button
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::spinner::SpinnerCircle;

#[component]
pub fn DemoSpinnerButton() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center">
            <Button attr:disabled=true>
                <SpinnerCircle />
                <span>"Loading..."</span>
            </Button>
            <Button variant=ButtonVariant::Outline attr:disabled=true>
                <SpinnerCircle />
                <span>"Please wait"</span>
            </Button>
            <Button variant=ButtonVariant::Secondary attr:disabled=true>
                <SpinnerCircle />
                <span>"Processing"</span>
            </Button>
        </div>
    }
}
```
