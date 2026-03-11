---
title: "Demo Use Lock Body Scroll"
name: "demo_use_lock_body_scroll"
cargo_dependencies: []
registry_dependencies: ["button", "card"]
type: "components:demos"
path: "demos/demo_use_lock_body_scroll.rs"
---

# Demo Use Lock Body Scroll

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_use_lock_body_scroll
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::hooks::use_lock_body_scroll::use_lock_body_scroll;
use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn DemoUseLockBodyScroll() -> impl IntoView {
    let scroll_locked = use_lock_body_scroll(false);

    let toggle_scroll_lock = move |_| {
        scroll_locked.update(|locked| *locked = !*locked);
    };

    view! {
        <Card class="mx-auto w-full max-w-md">
            <CardHeader>
                <CardTitle>"Body Scroll Lock Demo"</CardTitle>
                <CardDescription>
                    "Try scrolling the page. Click the button to lock/unlock body scrolling."
                </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="flex justify-between items-center">
                    <span class="text-sm font-medium">"Scroll Status:"</span>
                    <span class="py-1 px-2 text-sm rounded-full">
                        {move || if scroll_locked.get() { "🔒 Locked" } else { "🔓 Unlocked" }}
                    </span>
                </div>

                <Button on:click=toggle_scroll_lock class="w-full">
                    {move || if scroll_locked.get() { "Unlock Body Scroll" } else { "Lock Body Scroll" }}
                </Button>
            </CardContent>
        </Card>
    }
}
```
