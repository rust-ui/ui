---
title: "Demo Progress Controlled"
name: "demo_progress_controlled"
cargo_dependencies: []
registry_dependencies: ["progress"]
type: "components:demos"
path: "demos/demo_progress_controlled.rs"
---

# Demo Progress Controlled

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_progress_controlled
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::progress::Progress;

#[component]
pub fn DemoProgressControlled() -> impl IntoView {
    let value = RwSignal::new(50.0_f64);

    view! {
        <div class="flex overflow-hidden flex-col gap-4 w-60">
            <Progress value=value />
            <input
                type="range"
                min="0"
                max="100"
                step="1"
                prop:value=move || value.get()
                class="w-full accent-primary"
                on:input=move |e| {
                    let val = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                    value.set(val);
                }
            />
        </div>
    }
}
```
