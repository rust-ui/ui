---
title: "Demo Button Href"
name: "demo_button_href"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_button_href.rs"
---

# Demo Button Href

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_href
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoButtonHref() -> impl IntoView {
    view! {
        <div class="flex gap-10 p-4 rounded-lg border">
            <Button href="/dashboard">"Go to Dashboard"</Button>
            <Button href="/dashboard" variant=ButtonVariant::Ghost class="border-2 border-dashed">
                "Go to Dashboard (custom)"
            </Button>
            <Button href="https://ever-ui.com/" attr:rel="noopener" attr:target="_blank">
                "External Link"
            </Button>
        </div>
    }
}
```
