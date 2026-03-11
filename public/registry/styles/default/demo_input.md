---
title: "Demo Input"
name: "demo_input"
cargo_dependencies: []
registry_dependencies: ["input"]
type: "components:demos"
path: "demos/demo_input.rs"
---

# Demo Input

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::input::{Input, InputType};

// TODO Fix: Input type="number" can take "e" as a valid input

#[component]
pub fn DemoInput() -> impl IntoView {
    let name_signal = RwSignal::new(String::new());

    view! {
        <div class="space-y-4 w-full max-w-lg">
            <h2 class="text-2xl font-bold">Input Demo</h2>

            <Input placeholder="Default input" />
            <Input r#type=InputType::Email placeholder="Email input" />
            <Input r#type=InputType::Password placeholder="Password input" />
            <Input class="border-2 border-purple-500 focus:border-purple-700" placeholder="Custom styled input" />
            <Input r#type=InputType::Number placeholder="Number input" />

            // Number input with min/max/step
            <div class="pt-4 border-t">
                <p class="mb-2 text-sm text-muted-foreground">Number input with min/max/step:</p>
                <Input
                    r#type=InputType::Number
                    min="0"
                    max="10"
                    step="0.1"
                    placeholder="Score (0-10)"
                    title="Enter a score between 0 and 10"
                    class="w-32"
                />
            </div>

            // Two-way binding example
            <div class="pt-4 border-t">
                <p class="mb-2 text-sm text-muted-foreground">Two-way binding:</p>
                <Input placeholder="Type here..." bind_value=name_signal />
                <p class="mt-2 text-sm">"Value: " {move || name_signal.get()}</p>
            </div>
        </div>
    }
}
```
