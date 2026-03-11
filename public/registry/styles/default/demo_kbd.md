---
title: "Demo Kbd"
name: "demo_kbd"
cargo_dependencies: []
registry_dependencies: ["kbd"]
type: "components:demos"
path: "demos/demo_kbd.rs"
---

# Demo Kbd

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_kbd
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::kbd::{Kbd, KbdGroup};

#[component]
pub fn DemoKbd() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center">
            <KbdGroup>
                <Kbd>"⌘"</Kbd>
                <Kbd>"⇧"</Kbd>
                <Kbd>"⌥"</Kbd>
                <Kbd>"⌃"</Kbd>
            </KbdGroup>
            <KbdGroup>
                <Kbd>"Ctrl"</Kbd>
                <span>"+"</span>
                <Kbd>"B"</Kbd>
            </KbdGroup>
        </div>
    }
}
```
