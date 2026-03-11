---
title: "Demo Slider Controlled"
name: "demo_slider_controlled"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_slider_controlled.rs"
---

# Demo Slider Controlled

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_slider_controlled
```

## Component Code

```rust
use leptos::prelude::*;

#[component]
pub fn DemoSliderControlled() -> impl IntoView {
    let value = RwSignal::new(50_i32);

    view! {
        <div class="flex flex-col gap-4 w-72">
            <div class="flex justify-between items-center">
                <span class="text-sm font-medium">"Volume"</span>
                <span class="text-sm tabular-nums text-muted-foreground">{value}</span>
            </div>
            <input
                type="range"
                min="0"
                max="100"
                step="1"
                prop:value=move || value.get()
                class="overflow-hidden relative bg-transparent transition-all duration-100 ease-in-out appearance-none text-[1.5rem] w-[12.5em] text-primary active:cursor-grabbing"
                on:input=move |ev| {
                    if let Ok(v) = event_target_value(&ev).parse::<i32>() {
                        value.set(v);
                    }
                }
            />
        </div>
    }
}
```
