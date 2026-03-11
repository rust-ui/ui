---
title: "Demo Button Reactive"
name: "demo_button_reactive"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_button_reactive.rs"
---

# Demo Button Reactive

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_reactive
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButtonReactive() -> impl IntoView {
    let count_signal = RwSignal::new(0);
    let increment = move |_| *count_signal.write() += 1;

    view! { <Button on:click=increment>"Click Me: " {count_signal}</Button> }
}
```
