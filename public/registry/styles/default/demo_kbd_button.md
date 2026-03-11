---
title: "Demo Kbd Button"
name: "demo_kbd_button"
cargo_dependencies: []
registry_dependencies: ["button", "kbd"]
type: "components:demos"
path: "demos/demo_kbd_button.rs"
---

# Demo Kbd Button

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_kbd_button
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::kbd::Kbd;

#[component]
pub fn DemoKbdButton() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-4 items-center">
            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm class="pr-2">
                "Accept "
                <Kbd>"⏎"</Kbd>
            </Button>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm class="pr-2">
                "Cancel "
                <Kbd>"Esc"</Kbd>
            </Button>
        </div>
    }
}
```
