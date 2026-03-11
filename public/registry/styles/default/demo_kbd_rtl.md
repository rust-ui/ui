---
title: "Demo Kbd Rtl"
name: "demo_kbd_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "kbd"]
type: "components:demos"
path: "demos/demo_kbd_rtl.rs"
---

# Demo Kbd Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_kbd_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::kbd::{Kbd, KbdGroup};

#[component]
pub fn DemoKbdRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <div class="flex flex-col gap-4 items-center">
                <div class="flex gap-2 items-center text-sm text-muted-foreground">
                    <span>"بحث:"</span>
                    <KbdGroup>
                        <Kbd>"⌘"</Kbd>
                        <Kbd>"K"</Kbd>
                    </KbdGroup>
                </div>
                <div class="flex gap-2 items-center text-sm text-muted-foreground">
                    <span>"غامق:"</span>
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd>
                        <span>"+"</span>
                        <Kbd>"B"</Kbd>
                    </KbdGroup>
                </div>
            </div>
        </DirectionProvider>
    }
}
```
