---
title: "Demo Use Is Mobile"
name: "demo_use_is_mobile"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_use_is_mobile.rs"
---

# Demo Use Is Mobile

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_use_is_mobile
```

## Component Code

```rust
use icons::{Monitor, Smartphone};
use leptos::prelude::*;

use crate::components::hooks::use_is_mobile::use_is_mobile;

#[component]
pub fn DemoUseIsMobile() -> impl IntoView {
    let is_mobile = use_is_mobile();

    view! {
        <div class="flex flex-col gap-3 items-center">
            <div class="flex gap-2 items-center text-sm font-medium">
                {move || {
                    if is_mobile.get() {
                        view! {
                            <Smartphone class="size-4" />
                            <span>"Mobile"</span>
                        }
                            .into_any()
                    } else {
                        view! {
                            <Monitor class="size-4" />
                            <span>"Desktop"</span>
                        }
                            .into_any()
                    }
                }}
            </div>
            <p class="text-xs text-muted-foreground">"Resize the window below 768px to toggle."</p>
        </div>
    }
}
```
